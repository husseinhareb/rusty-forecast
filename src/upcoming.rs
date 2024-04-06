use serde::Deserialize;
use crate::config::{read_city_name,read_unit};
use crate::condition_icons::WeatherStatus;
use crate::condition_icons::map_weather_description_to_code;

#[derive(Deserialize)]
struct ForecastResponse {
    list: Vec<ForecastData>,
}

#[derive(Deserialize)]
struct ForecastData {
    dt_txt: String,
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

pub fn weather_forecast() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "2a33d8b44aa8d93d07feac453b4a79aa";
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

    let url = format!("http://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units={}", city_name, api_key,unit_type);

    let response = reqwest::blocking::get(&url)?.text()?;

    let data: serde_json::Value = serde_json::from_str(&response)?;

    if let Some(cod) = data["cod"].as_str() {
        if cod != "200" {
            println!("Error: {}", data["message"]);
            return Ok(());
        }
    } else {
        println!("Error: 'cod' field not found in response");
        return Ok(());
    }

    let forecast: ForecastResponse = serde_json::from_value(data)?;

    println!("{}", city_name);

    for forecast_data in forecast.list.iter().take(4) {
        let weather_description = match forecast_data.weather.get(0) {
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

        let date_time = &forecast_data.dt_txt;
        let date = &date_time.split(' ').collect::<Vec<&str>>()[0];

        let temp_box = format!("╔═════════════════════╗\n\
                                ║   Date: {}         ║\n\
                                ║   Weather: {}      ║\n\
                                ║   Temperature: {} °{}   ║\n\
                                ║   Humidity: {} %   ║\n\
                                ╚═════════════════════╝", 
                                date,
                                weather_status.icon(),
                                forecast_data.main.temp,
                                unit_value,
                                forecast_data.main.humidity
                               );

        println!("{}", temp_box);
    }

    Ok(())
}