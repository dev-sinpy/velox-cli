mod config;
mod server;
mod setup;
mod subprocess;

use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

use custom_error::custom_error;

custom_error! {pub VeloxError
    DependencyError{detail: String} = "{detail}",
    SubProcessError{detail: String} = "{detail}",
    IoError{source: std::io::Error} = "{source}",
    FileSystemError{source: fs_extra::error::Error} = "{source}",
    ServerError{detail: String} = "{detail}",
}

/// Creates a new velox project.
pub fn create_new_project(name: &str) -> Result<(), VeloxError> {
    use fs_extra::dir::{copy, CopyOptions};

    let current_dir = current_dir()?;
    let project_path = current_dir.join(name);

    // copy template to form a new project
    let template_path = Path::new("../velox-template"); // Todo: pull this dir from github
    if template_path.exists() {
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        copy(template_path, &project_path, &options)?;
    } else {
        // Todo: return more detailed error
        panic!("template does not exist");
    }

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
                Path::new(&project_path.join("velox-config.toml")),
                &setup_config,
            )?;

            // check if user has said to install dependencies
            if setup_config.install_dependencies {
                // check if npm or yarn is installed in system
                if !setup_config.package_manager.check_if_installed() {
                    panic!("Package manager is not Installed.");
                } else {
                    // install js dependencies
                    setup_config
                        .package_manager
                        .install_dependencies(Path::new(&project_path.join("web/")));
                }
            } else {
                println!("setup finished!");
            }
        }
        Err(err) => panic!("{:?}", err),
    };
    Ok(())
}

// Run project in debug or release mode.
pub fn run(release: bool) {
    let mut snowpack_process = server::start_dev_server();
    let arg = if release {
        "cargo-watch -s 'cargo run --release'"
    } else {
        "cargo-watch -s 'cargo run'"
    };
    let mut cargo_process = subprocess::exec(arg, Stdio::inherit(), Stdio::inherit());

    snowpack_process.wait().unwrap();
    cargo_process.wait().unwrap();
}
