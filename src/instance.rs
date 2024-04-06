use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;
use crate::config::read_city_name;
use crate::condition_icons::WeatherStatus;
use crate::config::read_unit;
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
    let city_name = match read_city_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error reading city name: {}", err);
            return Ok(());
        }
    };

    let unit_value = match read_unit() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error reading unit value: {}", err);
            return Ok(());
        }
    };
    
    let unit_type: &str;
    
    if unit_value == "C" {
        unit_type = "metric";
    } else {
        unit_type = "imperial";
    }
    
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}", city_name, api_key, unit_type);
    

    let response = Client::new().get(&url).send()?;

    let data: Value = serde_json::from_str(&response.text()?)?;

    if data["cod"] != 200 {
        println!("Error: {}", data["message"]);
        return Ok(());
    }

    let weather: WeatherResponse = serde_json::from_value(data)?;
    println!("{}",city_name);

    // Get the weather description from the response
    let weather_description = match weather.weather.get(0) {
        Some(desc) => &desc.description,
        None => {
            println!("No weather description available");
            return Ok(());
        }
    };

    // Match the weather description to get the corresponding WeatherStatus
    let weather_status = match weather_description.as_str() {
        "clear sky" => WeatherStatus::ClearSky,
        "few clouds" => WeatherStatus::FewClouds,
        "scattered clouds" => WeatherStatus::ScatteredClouds,
        "broken clouds" => WeatherStatus::BrokenClouds,
        "shower rain" => WeatherStatus::ShowerRain,
        "rain" => WeatherStatus::Rain,
        "thunderstorm" => WeatherStatus::Thunderstorm,
        "snow" => WeatherStatus::Snow,
        "mist" => WeatherStatus::Mist,
        _ => {
            println!("Unknown weather description");
            return Ok(());
        }
    };

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
