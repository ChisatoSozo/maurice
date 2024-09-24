use chrono::{DateTime, Duration, Local, Utc};
use std::{error::Error, fs, path::Path};

pub fn is_locked(file_path: &str, requestor_ip: &str) -> Result<(bool, bool), Box<dyn Error>> {
    let lock_file_path = format!("./files/{}.lock", file_path);
    let lock_file_path = Path::new(&lock_file_path);

    if !lock_file_path.exists() {
        return Ok((false, false));
    }

    let lock_content = fs::read_to_string(lock_file_path)?;
    let parts: Vec<&str> = lock_content.split_whitespace().collect();

    if parts.len() != 2 {
        return Err("Invalid lock file format".into());
    }

    let locked_ip = parts[0];
    let locked_time = parts[1].parse::<i64>()?;
    let locked_datetime = DateTime::<Utc>::from_timestamp(locked_time, 0)
        .ok_or("Invalid timestamp")?
        .with_timezone(&Local);
    let current_time = Local::now();

    if current_time.signed_duration_since(locked_datetime) > Duration::seconds(10) {
        // Lock is older than 10 seconds, delete it
        fs::remove_file(lock_file_path)?;
        Ok((false, false))
    } else {
        // File is locked
        Ok((true, locked_ip == requestor_ip))
    }
}

pub async fn edit_file(
    requestor_ip: String,
    file_path: String,
    file_content: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let (is_locked, owns_lock) = is_locked(&file_path, &requestor_ip)?;

    if is_locked && !owns_lock {
        return Err("File is locked by another user".into());
    }

    // Decode the base64 encoded file content and write it to the file
    let file_path_full = format!("./files/{}", file_path);
    let file_path_full = Path::new(&file_path_full);
    let parent = file_path_full.parent().ok_or("No parent directory")?;
    fs::create_dir_all(parent)?;
    fs::write(file_path_full, file_content)?;

    // Create a new lock file with the requestor_ip and the current time
    let lock_file_path = format!("./files/{}.lock", file_path);
    let lock_file_path = Path::new(&lock_file_path);
    fs::write(
        lock_file_path,
        format!("{} {}", requestor_ip, Utc::now().timestamp()),
    )?;

    Ok(())
}
