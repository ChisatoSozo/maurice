use std::{error::Error, fs, path::Path};

pub fn create_directory(file_path: String) -> Result<(), Box<dyn Error>> {
    let file_path_full = format!("./files/{}", file_path);
    let file_path_full = Path::new(&file_path_full);

    //does the dir already exist?
    if file_path_full.exists() {
        return Err("Directory already exists".into());
    }

    fs::create_dir_all(file_path_full)?;

    Ok(())
}
