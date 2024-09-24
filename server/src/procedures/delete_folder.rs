use std::{error::Error, fs, path::Path};

pub fn delete_folder(file_path: String) -> Result<(), Box<dyn Error>> {
    //is folder empty?
    let file_path_full = format!("./files/{}", file_path);
    let file_path_full = Path::new(&file_path_full);

    //does the folder already exist?
    if !file_path_full.exists() {
        return Err("Folder does not exist".into());
    }

    if !file_path_full.is_dir() {
        return Err("Path is not a folder".into());
    }

    let dir_entries = fs::read_dir(file_path_full)?;
    if dir_entries.count() > 0 {
        return Err("Folder is not empty".into());
    }

    fs::remove_dir(file_path_full)?;

    Ok(())
}
