use std::{error::Error, fs, path::Path};

use paperclip::actix::Apiv2Schema;
use serde::Serialize;

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct FilesAndDirectories {
    files: Vec<String>,
    directories: Vec<String>,
}

pub fn list_files_directory(path: &Path) -> Result<FilesAndDirectories, Box<dyn Error>> {
    println!("Listing files and directories in {:?}", path);
    let paths = fs::read_dir(path)?;
    let mut files = Vec::new();
    let mut directories = Vec::new();

    for path in paths {
        let path = path?.path();
        let path_str = path.to_str().unwrap().to_string();
        if path.is_dir() {
            directories.push(path_str);
        } else {
            files.push(path_str);
        }
    }

    Ok(FilesAndDirectories { files, directories })
}
