use std::process::Command;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufRead};
use std::path::Path;
use dirs;

//Function to get the default city according 
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
//Function to create the config file if not created
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

    Ok(())
}

//Function to write city name according to parameter into the config file
pub fn write_city_name(city_name: &str) -> io::Result<()> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("rusty-forecast").join("rusty-forecast.conf");

    if let Some(parent_dir) = file_path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }
    }
    let mut file = File::create(&file_path)?;

    file.write_all(format!("city   {}\n", city_name).as_bytes())?;

    Ok(())
}
// Function to write default city according to Timezone into the config file
pub fn write_def_city() -> io::Result<()> {
    // Get the default city
    let def_city = match default_city() {
        Ok(city) => city,
        Err(err) => return Err(err),
    };

    write_city_name(&def_city)
}

// Function to read city name from config file
pub fn read_city_name() -> io::Result<String> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("rusty-forecast").join("rusty-forecast.conf");

    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("city") {
            let city_name = line.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
            return Ok(city_name);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "City name not found"))
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

