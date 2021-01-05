use crate::subprocess;
use crate::VeloxError;

use dialoguer::console::Style;
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

        let mut process = subprocess::exec(command, Stdio::null(), Stdio::null());
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

    pub fn install_dependencies(&self, project_path: &Path) {
        let command = match self {
            PackageManager::Npm => format!("cd {};npm install", project_path.to_str().unwrap()),
            PackageManager::Yarn => format!("cd {};yarn", project_path.to_str().unwrap()),
        };

        let mut process = subprocess::exec(&command, Stdio::inherit(), Stdio::inherit());
        match process.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    panic!("Failed to install dependencies, please try again later.");
                }
            }
            Ok(None) => {
                println!("status not ready yet, let's really wait");
                match process.wait() {
                    Ok(status) => {
                        if !status.success() {
                            panic!("Failed to install dependencies, please try again later.");
                        }
                    }
                    Err(_err) => panic!("Failed to install dependencies, please try again later."),
                }
            }
            Err(_e) => panic!("Failed to install dependencies, please try again later."),
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
    println!("Create a new velox project");

    if Confirm::with_theme(&theme)
        .with_prompt("Do you want to use default config?")
        .interact()?
    {
        return Ok(SetupConfig::default());
    }

    let title = Input::with_theme(&theme)
        .with_prompt("App title")
        .default(project_name.parse().unwrap())
        .interact()?;

    let description = Input::with_theme(&theme)
        .with_prompt("App description")
        .default("".parse().unwrap())
        .interact()?;

    let package_manager = match Select::with_theme(&theme)
        .with_prompt("Installed package manager?")
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

    Ok(SetupConfig {
        title,
        description,
        package_manager,
        install_dependencies,
    })
}
