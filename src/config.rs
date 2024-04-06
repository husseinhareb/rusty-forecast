use std::fs::{self, File};
use std::path::Path;
use dirs;
use std::io::{self, prelude::*, BufRead};
use crate::city::default_city;

// Function to create the config file if not created
pub fn create_config() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().expect("Unable to determine config directory");
    let folder_path = config_dir.join("rusty-forecast");

    if !folder_exists(&folder_path) {
        fs::create_dir(&folder_path)?;
    }

    let file_path = folder_path.join("rusty-forecast.conf");

    if file_exists(&file_path) {
        return Ok(());
    }else{
        let _ = write_def_city();
    }

    Ok(())
}


// Function to read all configs
pub fn read_all_configs() -> io::Result<String> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("rusty-forecast").join("rusty-forecast.conf");

    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    
    let mut config_content = String::new();

    for line in reader.lines() {
        let line = line?;
        config_content.push_str(&line);
        config_content.push('\n');
        println!("{}",line);
    }

    if config_content.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Config content not found"));
    }

    Ok(config_content)
}


// Function to write city name according to parameter into the config file
pub fn write_city_name(city_name: &str) -> io::Result<()> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("rusty-forecast").join("rusty-forecast.conf");    

    let mut file_content = String::new();
    if file_path.exists() {
        let mut file = File::open(&file_path)?;
        file.read_to_string(&mut file_content)?;
    }

    let mut updated_content = String::new();
    let mut city_found = false;
    for line in file_content.lines() {
        if line.trim().starts_with("city") {
            city_found = true;
            updated_content.push_str(&format!("city {}\n", city_name));
        } else {
            updated_content.push_str(&line);
            updated_content.push('\n');
        }
    }

    if !city_found {
        updated_content.push_str(&format!("city {}\n", city_name));
    }

    let mut file = File::create(&file_path)?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
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


// Function to write unit value according to parameter into the config file
pub fn write_unit(unit_value: &char) -> io::Result<()> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("rusty-forecast").join("rusty-forecast.conf");   

    let mut file_content = String::new();
    if file_path.exists() {
        let mut file = File::open(&file_path)?;
        file.read_to_string(&mut file_content)?;
    }

    let mut updated_content = String::new();
    let mut unit_found = false;
    for line in file_content.lines() {
        if line.trim().starts_with("unit") {
            unit_found = true;
            updated_content.push_str(&format!("unit {}\n", unit_value));
        } else {
            updated_content.push_str(&line);
            updated_content.push('\n');
        }
    }

    if !unit_found {
        updated_content.push_str(&format!("unit {}\n", unit_value));
    }

    let mut file = File::create(&file_path)?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

pub fn read_unit() -> io::Result<String> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("rusty-forecast").join("rusty-forecast.conf");

    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("unit") {
            let city_name = line.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
            return Ok(city_name);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "unit name not found"))
}

// Function to write default city according to Timezone into the config file
pub fn write_def_city() -> io::Result<()> {
    let def_city = match default_city() {
        Ok(city) => city,
        Err(err) => return Err(err),
    };

    write_city_name(&def_city)
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

