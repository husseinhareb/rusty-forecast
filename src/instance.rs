use serde::Deserialize;
use crate::config::{read_city_name, read_unit};
use crate::condition_icons::WeatherStatus;
use crate::condition_icons::map_weather_description_to_code;
use chrono::{NaiveDateTime, TimeZone, Local};

#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherData,
    weather: Vec<WeatherDescription>,
    sys: SysData,

}

#[derive(Deserialize)]
struct WeatherDescription {
    description: String,
}

#[derive(Deserialize)]
struct WeatherData {
    temp: f32,
    humidity: f32,
    pressure: f32,
    feels_like: f32,
    temp_max: f32,
    temp_min: f32,
}

#[derive(Deserialize)]
struct SysData {
    sunrise: i64,
    sunset: i64,
}


fn fetch_weather_data() -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let api_key = "2a33d8b44aa8d93d07feac453b4a79aa";

    let city_name = read_city_name()?;
    let unit_value = read_unit()?;

    let unit_type = if unit_value == "C" {
        "metric"
    } else {
        "imperial"
    };

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        city_name, api_key, unit_type
    );

    let response: serde_json::Value = reqwest::blocking::get(&url)?.json()?;
    if response["cod"] != 200 {
        return Err(format!("Error: {}", response["message"]).into());
    }
    serde_json::from_value(response).map_err(Into::into)
}

pub fn weather_now() -> Result<(), Box<dyn std::error::Error>> {
    let weather = match fetch_weather_data() {
        Ok(weather) => weather,
        Err(err) => {
            eprintln!("Error fetching weather data: {}", err);
            return Ok(());
        }
    };

    let weather_description = match weather.weather.get(0) {
        Some(desc) => &desc.description,
        None => {
            println!("No weather description available");
            return Ok(());
        }
    };

    let weather_code = match map_weather_description_to_code(&weather_description) {
        Some(code) => code,
        None => {
            println!("Unknown weather description");
            return Ok(());
        }
    };

    let weather_status = match WeatherStatus::from_weather_code(weather_code) {
        Some(status) => status,
        None => {
            println!("Unsupported weather code");
            return Ok(());
        }
    };

    let temp_box = format!(
        "╔═════════════════════╗\n\
         ║        {}         ║\n\
         ║        {}      ║\n\
         ║        {} °{}       ║\n\
         ║        {} %       ║\n\
         ╚═════════════════════╝",
        weather_status.icon(),
        weather.weather[0].description,
        weather.main.temp,
        read_unit()?,
        weather.main.humidity
    );

    println!("{}", temp_box);

    Ok(())
}


pub fn weather_details() -> Result<(), Box<dyn std::error::Error>> {
    let weather = fetch_weather_data()?;
    println!("City: {}", read_city_name()?);
    println!("Temperature: {}°{}", weather.main.temp, read_unit()?);
    println!("Feels Like: {}°{}", weather.main.feels_like, read_unit()?);
    println!("Weather Description: {}", weather.weather[0].description);
    println!("Minimum Temperature: {}°{}", weather.main.temp_min, read_unit()?);
    println!("Maximum Temperature: {}°{}", weather.main.temp_max, read_unit()?);
    println!("Humidity: {}%", weather.main.humidity);
    println!("Pressure: {}", weather.main.pressure);
    println!("Sunrise: {}", unix_timestamp_to_local_time(weather.sys.sunrise));
    println!("Sunset: {}", unix_timestamp_to_local_time(weather.sys.sunset));

    Ok(())
}

fn unix_timestamp_to_local_time(timestamp: i64) -> String {
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);

    let local_datetime = Local.from_utc_datetime(&naive_datetime);

    let formatted_time = local_datetime.format("%H:%M:%S").to_string();

    formatted_time
}