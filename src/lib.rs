mod config;
mod setup;
mod subprocess;
mod template;

// use confy::ConfyError;
use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{self, Stdio};

use custom_error::custom_error;

custom_error! {pub VeloxError
    // ConfigError{source: ConfyError} = "{source}",
    InstallationError{detail: String} = "{detail}",
    SetupError{detail: String} = "{detail}",
    DependencyError{detail: String} = "{detail}",
    JSONError{source: serde_json::error::Error} = "{source}",
    SubProcessError{detail: String} = "{detail}",
    IoError{source: std::io::Error} = "{source}",
    ServerError{detail: String} = "{detail}",
}

/// Creates a new velox project.
pub fn create_new_project(name: &str) -> Result<(), VeloxError> {
    let current_dir = current_dir()?;
    let project_path = current_dir.join(name);

    // Create a new template folder with project name
    template::create_template(&project_path)?;

    // edit fields in Cargo.toml file with user preferences
    let file_content = fs::read_to_string(&project_path.join("Cargo.toml"))?;
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(project_path.join("Cargo.toml"))?;

    let updated_content = file_content.replace("{{project_name}}", name);
    file.write_all(updated_content.as_bytes())?;

    // Begin setup
    match setup::begin_setup(name) {
        // Check if user has entered correct data
        Ok(setup_config) => {
            // set velox-config file with user data
            config::set_config(
                Path::new(&project_path.join("velox-config.json")),
                &setup_config,
            )?;

            // check if user has said to install dependencies
            if setup_config.install_dependencies {
                // check if npm or yarn is installed in system
                if !setup_config.package_manager.check_if_installed() {
                    Err(VeloxError::DependencyError {
                        detail: String::from("Package manager is not Installed."),
                    })
                } else {
                    // install js dependencies
                    setup_config
                        .package_manager
                        .install_dependencies(Path::new(&project_path.join("web/")))?;
                    Ok(())
                }
            } else {
                Ok(())
            }
        }
        Err(err) => Err(VeloxError::SetupError {
            detail: err.to_string(),
        }),
    }
}

// Run project in debug or release mode.
pub fn run() -> Result<(), VeloxError> {
    let snowpack_command = match config::load_config()?.package_manager.as_str() {
        "npm" => "npx dev",
        "yarn" => "yarn run dev",
        _ => {
            return Err(VeloxError::SubProcessError {
                detail: "Invalid velox config".to_string(),
            })
        }
    };
    let mut snowpack_process =
        subprocess::exec(snowpack_command, "web/", Stdio::inherit(), Stdio::inherit());
    let cargo_command = "cargo-watch -s 'cargo run debug'";
    let mut cargo_process =
        subprocess::exec(cargo_command, ".", Stdio::inherit(), Stdio::inherit());

    if let Err(err) = snowpack_process.wait() {
        return Err(VeloxError::SubProcessError {
            detail: err.to_string(),
        });
    };
    if let Err(err) = cargo_process.wait() {
        return Err(VeloxError::SubProcessError {
            detail: err.to_string(),
        });
    };

    Ok(())
}

pub fn build() -> Result<(), VeloxError> {
    let config = config::load_config()?;
    let snowpack_command = match config.package_manager.as_str() {
        "npm" => "npx build",
        "yarn" => "yarn run build",
        _ => {
            return Err(VeloxError::SubProcessError {
                detail: "Invalid velox config".to_string(),
            })
        }
    };
    let snowpack_process = if cfg!(target_os = "windows") {
        process::Command::new("cmd")
            .current_dir("web/")
            .args(&["/C", snowpack_command])
            .status()?
    } else {
        process::Command::new("sh")
            .current_dir("web/")
            .args(&["-c", snowpack_command])
            .status()?
    };

    if !snowpack_process.success() {
        panic!("BundlerError: Failed to build assets.");
    } else {
        velox_bundler::bundle_binary().unwrap();
        run_cleanup(config.build_dir)?;
    }
    Ok(())
}

fn run_cleanup<T: std::convert::AsRef<std::path::Path>>(path: T) -> Result<(), VeloxError> {
    fs::remove_dir_all(path)?;
    Ok(())
}
