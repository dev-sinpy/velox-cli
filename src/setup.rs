use crate::subprocess;
use crate::VeloxError;

use dialoguer::console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::path::Path;
use std::process::Stdio;

#[derive(Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
}

impl PackageManager {
    pub fn check_if_installed(&self) -> bool {
        let command = match self {
            PackageManager::Npm => "npm --version",
            PackageManager::Yarn => "yarn --version",
        };

        let mut process = subprocess::exec(command, ".", Stdio::null(), Stdio::null());
        match process.try_wait() {
            Ok(Some(status)) => status.success(),
            Ok(None) => {
                println!("status not ready yet, let's really wait");
                let status = process.wait().unwrap();
                status.success()
            }
            Err(_e) => false,
        }
    }

    pub fn install_dependencies(&self, project_path: &Path) -> Result<(), VeloxError> {
        let command = match self {
            PackageManager::Npm => "npm install".to_string(),
            PackageManager::Yarn => "yarn".to_string(),
        };

        let mut process =
            subprocess::exec(&command, project_path, Stdio::inherit(), Stdio::inherit());
        match process.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    Err(VeloxError::InstallationError {
                        detail: format!(
                            "{}Failed to install dependencies, please try again later or install them manually.",
                            style("Error: ").red().bold()
                        ),
                    })
                } else {
                    Ok(())
                }
            }
            Ok(None) => {
                println!("status not ready yet, let's really wait");
                match process.wait() {
                    Ok(status) => {
                        if !status.success() {
                            Err(VeloxError::InstallationError {
                                detail: String::from(
                                    "Failed to install dependencies, please try again later or install them manually."
                                ),
                            })
                        } else {
                            Ok(())
                        }
                    }
                    Err(_err) => Err(VeloxError::InstallationError {
                        detail: String::from(
                            "Failed to install dependencies, please try again later or install them manually."
                        ),
                    }),
                }
            }
            Err(err) => Err(VeloxError::SubProcessError {
                detail: err.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct SetupConfig {
    pub title: String,
    pub description: String,
    pub package_manager: PackageManager,
    pub install_dependencies: bool,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            title: String::from("hello-world"),
            description: String::new(),
            package_manager: PackageManager::Npm,
            install_dependencies: true,
        }
    }
}

pub fn begin_setup(project_name: &str) -> Result<SetupConfig, VeloxError> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    print!("\x1B[2J\x1B[1;1H");
    println!(
        "{:4}{}",
        "",
        style("Create a new velox project.\n").green().bold(),
    );

    let title = Input::with_theme(&theme)
        .with_prompt("App title")
        .default(project_name.parse().unwrap())
        .interact()?;

    let description = Input::with_theme(&theme)
        .with_prompt("App description")
        .default("".parse().unwrap())
        .interact()?;

    let package_manager = match Select::with_theme(&theme)
        .with_prompt("Select package manager")
        .default(0)
        .item("Npm")
        .item("Yarn")
        .interact()?
    {
        0 => PackageManager::Npm,
        1 => PackageManager::Yarn,
        _ => PackageManager::Npm,
    };

    let install_dependencies = Confirm::with_theme(&theme)
        .with_prompt("Do you want to install dependencies now?")
        .default(true)
        .interact()?;

    println!(
        "{:4}{:} ({})",
        "\n",
        style("Successfully created new project").green(),
        style(project_name).cyan().bold()
    );

    Ok(SetupConfig {
        title,
        description,
        package_manager,
        install_dependencies,
    })
}
