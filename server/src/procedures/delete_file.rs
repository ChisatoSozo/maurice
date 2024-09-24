use std::{error::Error, fs, path::Path};

use super::edit_file::is_locked;

pub fn delete_file(requestor_ip: String, file_path: String) -> Result<(), Box<dyn Error>> {
    let (is_locked, owns_lock) = is_locked(&file_path, &requestor_ip)?;

    if is_locked && !owns_lock {
        return Err("File is locked by another user".into());
    }

    // Decode the base64 encoded file content and write it to the file
    let file_path_full = format!("./files/{}", file_path);
    let file_path_full = Path::new(&file_path_full);

    //does the file already exist?
    if !file_path_full.exists() {
        return Err("File does not exist".into());
    }

    fs::remove_file(file_path_full)?;

    Ok(())
}
