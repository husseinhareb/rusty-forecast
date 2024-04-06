// instance.rs

use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;
use crate::config::{read_city_name, read_unit};
use crate::condition_icons::WeatherStatus;
use crate::condition_icons::map_weather_description_to_code;
#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherData,
    weather: Vec<WeatherDescription>,
}

#[derive(Deserialize)]
struct WeatherDescription {
    description: String,
}

#[derive(Deserialize)]
struct WeatherData {
    temp: f32,
    humidity: f32,
}



pub fn weather_now() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "2a33d8b44aa8d93d07feac453b4a79aa";

    // Read city name from config
    let city_name = match read_city_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error reading city name: {}", err);
            return Ok(());
        }
    };

    // Read unit value from config
    let unit_value = match read_unit() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error reading unit value: {}", err);
            return Ok(());
        }
    };
    
    // Determine unit type based on unit value
    let unit_type = if unit_value == "C" {
        "metric"
    } else {
        "imperial"
    };
    
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}", 
                      city_name, api_key, unit_type);

    let response = Client::new().get(&url).send()?;

    let data: Value = serde_json::from_str(&response.text()?)?;

    if data["cod"] != 200 {
        println!("Error: {}", data["message"]);
        return Ok(());
    }

    let weather: WeatherResponse = serde_json::from_value(data)?;
    println!("{}", city_name);
    println!("{}", weather.weather[0].description);

    // Get the weather description from the response
    let weather_description = match weather.weather.get(0) {
        Some(desc) => &desc.description,
        None => {
            println!("No weather description available");
            return Ok(());
        }
    };

    // Map weather description to weather code
    let weather_code = match map_weather_description_to_code(&weather_description) {
        Some(code) => code,
        None => {
            println!("Unknown weather description");
            return Ok(());
        }
    };

    // Get the WeatherStatus corresponding to the weather code
    let weather_status = match WeatherStatus::from_weather_code(weather_code) {
        Some(status) => status,
        None => {
            println!("Unsupported weather code");
            return Ok(());
        }
    };

    // Print weather information
    let temp_box = format!("╔═════════════════════╗\n\
                            ║        {}         ║\n\
                            ║        {}      ║\n\
                            ║        {} °{}       ║\n\
                            ║        {} %       ║\n\
                            ╚═════════════════════╝", 
                            weather_status.icon(),
                            weather.weather[0].description,
                            weather.main.temp,
                            unit_value,
                            weather.main.humidity
                           );

    println!("{}", temp_box);
    
    Ok(())
}
