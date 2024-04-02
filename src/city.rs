use std::process::Command;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use dirs;

pub fn default_city() -> Result<String, std::io::Error> {
    let output = Command::new("sh")
                         .arg("-c")
                         .arg("timedatectl | awk '/Time zone/ {split($3, a, \"/\"); print a[2]}'")
                         .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("{}", result);
        Ok(result)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Command execution failed"))
    }
}

pub fn create_config() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().expect("Unable to determine config directory");
    let folder_path = config_dir.join("rusty-forecast");

    if !folder_exists(&folder_path) {
        fs::create_dir(&folder_path)?;
    }

    let file_path = folder_path.join("rusty-forecast.conf");

    if file_exists(&file_path) {
        return Ok(());
    }

    let mut file = File::create(&file_path)?;

    Ok(())
}

pub fn load_def_city() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().expect("Unable to determine config directory");
    let folder_path = config_dir.join("rusty-forecast");
    let file_path = folder_path.join("rusty-forecast.conf");

    if !file_exists(&file_path) {
        println!("Config file does not exist: {:?}", file_path);
        return Ok(());
    }

    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Contents of the config file: {}", contents);

    Ok(())
}

// Function to check if a folder exists
fn folder_exists(folder_path: &Path) -> bool {
    if let Ok(metadata) = std::fs::metadata(folder_path) {
        metadata.is_dir()
    } else {
        false
    }
}

// Function to check if a file exists
fn file_exists(file_path: &Path) -> bool {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}

