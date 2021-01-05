use crate::setup::SetupConfig;
use crate::VeloxError;

use std::path::Path;

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
    let updated_content = file_content
        .replace("{{app_title}}", &config.title)
        .replace("{{app_description}}", &config.description);
    file.write_all(updated_content.as_bytes())?;
    Ok(())
}
