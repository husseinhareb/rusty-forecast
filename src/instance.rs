use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;
use crate::city::read_city_name;
use crate::condition_icons::WeatherStatus;

#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherData,
    weather: Vec<WeatherDescription>,
}

#[derive(Deserialize)]
struct WeatherData {
    temp: f32,
    humidity: f32,
}

#[derive(Deserialize)]
struct WeatherDescription {
    description: String,
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
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city_name, api_key);

    let response = Client::new().get(&url).send()?;

    let data: Value = serde_json::from_str(&response.text()?)?;

    if data["cod"] != 200 {
        println!("Error: {}", data["message"]);
        return Ok(());
    }

    let weather: WeatherResponse = serde_json::from_value(data)?;

    let weather_status = match weather.weather.get(0) {
        Some(description) => match description.description.as_str() {
            "clear sky" => WeatherStatus::clear_sky,
            "few clouds" => WeatherStatus::few_clouds,
            "scattered clouds" => WeatherStatus::scattered_clouds,
            "broken clouds" => WeatherStatus::broken_clouds,
            "shower rain" => WeatherStatus::shower_rain,
            "rain" => WeatherStatus::rain,
            "thunderstorm" => WeatherStatus::thunderstorm,
            "snow" => WeatherStatus::snow,
            "mist" => WeatherStatus::mist,
            _ => WeatherStatus::mist, // Default to mist if unknown description
        },
        None => WeatherStatus::mist, // Default to mist if no weather description available
    };

    let temp_box = format!("╔═══════════════════════════╗\n║ Temperature: {:>6}°C\n║ Description: {:>12}    {}\n╚═══════════════════════════╝", 
                           weather.main.temp, 
                           weather.weather[0].description, 
                           weather_status.icon());

    println!("{}", temp_box);
    
    Ok(())
}
