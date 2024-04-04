use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;
use crate::city::read_city_name;

#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherData,
    weather: Vec<WeatherDescription>,
    wind: WindData,
    visibility: Option<u32>,
    clouds: Option<CloudData>,
    sys: SysData,
}

#[derive(Deserialize)]
struct WeatherDescription {
    description: String,
}

#[derive(Deserialize)]
struct WindData {
    speed: f32,
    deg: f32,
}

#[derive(Deserialize)]
struct CloudData {
    all: f32,
}

#[derive(Deserialize)]
struct SysData {
    sunrise: u64,
    sunset: u64,
}


#[derive(Deserialize)]
struct WeatherData {
    temp: f32,
    humidity: f32,
    description: String,
    wind_speed: f32,
    visibility: Option<f32>,
    clouds: Option<f32>,
    sunrise: Option<u64>,
    sunset: Option<u64>,
    pressure: Option<f32>,
    temp_min: Option<f32>,
    temp_max: Option<f32>,
    feels_like: Option<f32>,
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

    let temp_box = format!("╔═══════════════════════════╗\n║ Temperature: {:>6}°C                                   ║\n╚═══════════════════════════╝", weather.main.temp);

    println!("{}", temp_box);
    
    Ok(())
}
