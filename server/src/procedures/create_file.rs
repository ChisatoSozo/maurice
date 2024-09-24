use std::{error::Error, fs, path::Path};

pub fn create_file(file_path: String) -> Result<(), Box<dyn Error>> {
    let file_path_full = format!("./files/{}", file_path);
    let file_path_full = Path::new(&file_path_full);

    //does the file already exist?
    if file_path_full.exists() {
        return Err("File already exists".into());
    }

    let parent = file_path_full.parent().ok_or("No parent directory")?;
    fs::create_dir_all(parent)?;

    // Create the file
    fs::write(file_path_full, "")?;

    Ok(())
}
