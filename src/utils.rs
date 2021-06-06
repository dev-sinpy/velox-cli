use crate::{config, Result};

use std::fs;
use std::io::Write;

pub fn run_cleanup<T: std::convert::AsRef<std::path::Path>>(path: T) -> Result<()> {
    if path.as_ref().is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn move_artifacts(config: &config::VeloxConfig) -> Result<()> {
    println!("moving artifacts");
    fs::rename(
        format!("./target/release/{}.exe", config.name),
        format!("./dist/{}.exe", config.name),
    )?;
    if cfg!(target_arch = "x86") {
        let dll = include_bytes!("../dll/x86/WebView2Loader.dll");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("./dist/WebView2Loader.dll")?;
        file.write_all(dll)?;
    } else if cfg!(target_arch = "x86_64") {
        let dll = include_bytes!("../dll/x64/WebView2Loader.dll");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("./dist/WebView2Loader.dll")?;
        file.write_all(dll)?;
    } else if cfg!(target_arch = "aarch_64") {
        let dll = include_bytes!("../dll/arm64/WebView2Loader.dll");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("./dist/WebView2Loader.dll")?;
        file.write_all(dll)?;
    } else {
        panic!("Unsupported Arch: Your current cpu architecture is not supported.");
    }
    Ok(())
}
