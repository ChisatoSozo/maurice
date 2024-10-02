use std::{error::Error, fs, path::Path, time::SystemTime};

pub fn list_files(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Listing files in {:?}", path);
    let paths = fs::read_dir(path)?;
    let mut files = Vec::new();

    for entry in paths {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?;
            let path_str = path.to_str().unwrap().to_string();
            files.push((path_str, modified));
        }
    }

    // Sort files by modification time (most recent first)
    files.sort_by(|a, b| b.1.cmp(&a.1));

    // Extract only the file paths
    let sorted_files: Vec<String> = files.into_iter().map(|(path, _)| path).collect();

    Ok(sorted_files)
}
