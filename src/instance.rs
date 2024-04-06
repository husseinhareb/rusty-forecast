// instance.rs

use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;
use crate::config::{read_city_name, read_unit};
use crate::condition_icons::WeatherStatus;

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

// Mapping from weather description to weather code
fn map_weather_description_to_code(description: &str) -> Option<u16> {
    match description {
        "thunderstorm with light rain" => Some(200),
        "thunderstorm with rain" => Some(201),
        "thunderstorm with heavy rain" => Some(202),
        "light thunderstorm" => Some(210),
        "thunderstorm" => Some(211),
        "heavy thunderstorm" => Some(212),
        "ragged thunderstorm" => Some(221),
        "thunderstorm with light drizzle" => Some(230),
        "thunderstorm with drizzle" => Some(231),
        "thunderstorm with heavy drizzle" => Some(232),
        "light intensity drizzle" => Some(300),
        "drizzle" => Some(301),
        "heavy intensity drizzle" => Some(302),
        "light intensity drizzle rain" => Some(310),
        "drizzle rain" => Some(311),
        "heavy intensity drizzle rain" => Some(312),
        "shower rain and drizzle" => Some(313),
        "heavy shower rain and drizzle" => Some(314),
        "shower drizzle" => Some(321),
        "light rain" => Some(500),
        "moderate rain" => Some(501),
        "heavy intensity rain" => Some(502),
        "very heavy rain" => Some(503),
        "extreme rain" => Some(504),
        "freezing rain" => Some(511),
        "light intensity shower rain" => Some(520),
        "shower rain" => Some(521),
        "heavy intensity shower rain" => Some(522),
        "ragged shower rain" => Some(531),
        "light snow" => Some(600),
        "snow" => Some(601),
        "heavy snow" => Some(602),
        "sleet" => Some(611),
        "light shower sleet" => Some(612),
        "shower sleet" => Some(613),
        "light rain and snow" => Some(615),
        "rain and snow" => Some(616),
        "light shower snow" => Some(620),
        "shower snow" => Some(621),
        "heavy shower snow" => Some(622),
        "mist" => Some(701),
        "smoke" => Some(711),
        "haze" => Some(721),
        "sand/dust whirls" => Some(731),
        "fog" => Some(741),
        "sand" => Some(751),
        "dust" => Some(761),
        "volcanic ash" => Some(762),
        "squalls" => Some(771),
        "tornado" => Some(781),
        "scattered clouds" => Some(802),
        _ => None,
    }
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
