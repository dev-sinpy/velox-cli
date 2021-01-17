use crate::VeloxError;
use include_dir::{include_dir, Dir};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn create_template(path: &Path) -> Result<(), VeloxError> {
    // Create new folder with project name
    create_folder(path)?;

    static PROJECT_DIR: Dir = include_dir!("velox-template/");
    copy_files(&PROJECT_DIR, path)?;
    Ok(())
}

fn copy_files(dir: &Dir, path: &Path) -> Result<(), VeloxError> {
    for file in dir.files() {
        let mut new_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.join(file.path().file_name().unwrap()))
            .unwrap();
        let content = file.contents_utf8().unwrap().as_bytes();
        new_file.write_all(&content)?;
        // println!("{}", file);
    }

    for folder in dir.dirs() {
        let resolved_path = &path.join(folder.path().file_name().unwrap());
        create_folder(resolved_path)?;
        copy_files(folder, resolved_path)?;
    }
    Ok(())
}

fn create_folder(path: &Path) -> Result<(), VeloxError> {
    fs::DirBuilder::new().create(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_template_folder() {
        create_template(Path::new(".test")).unwrap();
        let template = Path::new(".test");
        if template.exists() {
            fs::remove_dir_all(template).unwrap();
            assert!(true);
        }
    }
}
