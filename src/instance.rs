// instance.rs

use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;
use crate::city::read_city_name;
#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherData,
}

#[derive(Deserialize)]
struct WeatherData {
    temp: f32,
    humidity: f32,
}


pub fn weather_now() -> Result<(), Box<dyn std::error::Error>> {
    // Your API key from OpenWeatherMap
    let api_key = "2a33d8b44aa8d93d07feac453b4a79aa";
    // City name for which you want to fetch weather data
    let city_name = match read_city_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error reading city name: {}", err);
            return Ok(()); // or handle the error accordingly
        }
    };
    // OpenWeatherMap API URL
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city_name, api_key);

    // Make a GET request to OpenWeatherMap API
    let response = Client::new().get(&url).send()?;

    // Deserialize JSON response
    let data: Value = serde_json::from_str(&response.text()?)?;

    // Check if the response contains an error
    if data["cod"] != 200 {
        println!("Error: {}", data["message"]);
        return Ok(());
    }

    // Deserialize weather data
    let weather: WeatherResponse = serde_json::from_value(data)?;

    // Display weather information
    println!("Temperature: {}°C", weather.main.temp);
    println!("Humidity: {}%", weather.main.humidity);

    Ok(())
}
