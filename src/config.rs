use crate::setup::{PackageManager, SetupConfig};
use crate::VeloxError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct VeloxConfig {
    ///title of the app
    pub title: String,
    pub description: String,
    pub debug: bool,
    pub permissions: Vec<String>,
    pub dev_server_url: String,
    pub package_manager: String,
}

impl ::std::default::Default for VeloxConfig {
    fn default() -> Self {
        Self {
            title: String::from("None"),
            description: String::from("None"),
            debug: true,
            permissions: vec![],
            dev_server_url: String::from("http://localhost:8889"),
            package_manager: String::from("npm"),
        }
    }
}

pub fn set_config(config_path: &Path, config: &SetupConfig) -> Result<(), VeloxError> {
    use std::fs;
    use std::io::Write;

    let file_content = fs::read_to_string(config_path).unwrap();
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(config_path)
        .unwrap();
    let package_manager = match config.package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Yarn => "yarn",
    };
    let updated_content = file_content
        .replace("{{app_title}}", &config.title)
        .replace("{{app_description}}", &config.description)
        .replace("{{package_manager}}", package_manager);

    file.write_all(updated_content.as_bytes())?;
    Ok(())
}

pub fn load_config() -> Result<VeloxConfig, VeloxError> {
    let config: VeloxConfig = confy::load_path("velox-config.toml")?;
    Ok(config)
}
