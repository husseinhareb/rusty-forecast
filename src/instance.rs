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
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city_name, api_key);

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
        "clear sky" => WeatherStatus::clear_sky,
        "few clouds" => WeatherStatus::few_clouds,
        "scattered clouds" => WeatherStatus::scattered_clouds,
        "broken clouds" => WeatherStatus::broken_clouds,
        "shower rain" => WeatherStatus::shower_rain,
        "rain" => WeatherStatus::rain,
        "thunderstorm" => WeatherStatus::thunderstorm,
        "snow" => WeatherStatus::snow,
        "mist" => WeatherStatus::mist,
        _ => {
            println!("Unknown weather description");
            return Ok(());
        }
    };

    let temp_box = format!("╔═════════════════════╗\n\
                            ║        {}         ║\n\
                            ║        {}  °C     ║\n\
                            ║        {} %       ║\n\
                            ║        {}         ║\n\
                            ╚═════════════════════╝", 
                            weather_status.icon(),
                            weather.weather[0].description,
                            weather.main.temp,
                            weather.main.humidity
                           );

    println!("{}", temp_box);
    
    Ok(())
}
