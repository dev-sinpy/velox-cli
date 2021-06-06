mod config;
mod setup;
mod subprocess;
mod template;
mod utils;

use crate::utils::*;

use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{self, Stdio};

use custom_error::custom_error;

pub type Result<T> = std::result::Result<T, Error>;

custom_error! {pub Error
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
pub fn create_new_project(name: &str) -> Result<()> {
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
                Path::new(&project_path.join("velox.conf.json")),
                &setup_config,
            )?;

            // check if user has said to install dependencies
            if setup_config.install_dependencies {
                // check if npm or yarn is installed in system
                if !setup_config.package_manager.check_if_installed() {
                    Err(Error::DependencyError {
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
        Err(err) => Err(Error::SetupError {
            detail: err.to_string(),
        }),
    }
}

// Run project in debug or release mode.
pub fn run() -> Result<()> {
    let snowpack_command = match config::load_config()?.package_manager.as_str() {
        "npm" => "npm run dev",
        "yarn" => "yarn run dev",
        _ => {
            return Err(Error::SubProcessError {
                detail: "Invalid velox config".to_string(),
            })
        }
    };
    let mut snowpack_process =
        subprocess::exec(snowpack_command, "web/", Stdio::inherit(), Stdio::inherit());
    let cargo_command = "cargo-watch -x run";
    let mut cargo_process =
        subprocess::exec(cargo_command, ".", Stdio::inherit(), Stdio::inherit());

    if let Err(err) = snowpack_process.wait() {
        return Err(Error::SubProcessError {
            detail: err.to_string(),
        });
    }
    if let Err(err) = cargo_process.wait() {
        return Err(Error::SubProcessError {
            detail: err.to_string(),
        });
    };

    Ok(())
}

pub fn build() -> Result<()> {
    let config = config::load_config()?;
    let snowpack_command = match config.package_manager.as_str() {
        "npm" => "npm run build",
        "yarn" => "yarn run build",
        _ => {
            return Err(Error::SubProcessError {
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
    }

    let cargo_process = if cfg!(target_os = "windows") {
        process::Command::new("cmd")
            .args(&["/C", "cargo build --release"])
            .status()?
    } else {
        process::Command::new("sh")
            .args(&["-c", "cargo build --release"])
            .status()?
    };

    if !cargo_process.success() {
        run_cleanup(&config.build_dir)?;
        panic!("BundlerError: Failed to build binary.");
    } else {
        if cfg!(target_os = "windows") {
            move_artifacts(&config).unwrap();
        }
        bundle_binary(&config).unwrap();
    }

    Ok(())
}

fn bundle_binary(config: &config::VeloxConfig) -> Result<()> {
    if cfg!(target_os = "windows") {
        // if true {
        let script = include_str!("../scripts/create_msi.py");
        {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("./create_msi.py")?;

            file.write_all(script.as_bytes())?;
        }
        let command = "python create_msi.py velox.conf.json";
        let bunding_process = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .args(&["/C", command])
                .status()?
        } else {
            process::Command::new("sh")
                .args(&["-c", command])
                .status()?
        };
        if !bunding_process.success() {
            run_cleanup("./create_msi.py")?;
            run_cleanup(&config.build_dir)?;
            panic!("BundlerError: Failed to build installer.");
        }
    } else {
        velox_bundler::bundle_binary().unwrap();
    }
    if cfg!(target_os = "windows") {
        run_cleanup("./create_msi.py")?;
    }
    run_cleanup(&config.build_dir)?;
    Ok(())
}
