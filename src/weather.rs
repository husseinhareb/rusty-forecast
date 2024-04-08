use serde::Deserialize;
use crate::config::{read_city_name, read_unit};
use crate::condition_icons::WeatherStatus;
use crate::condition_icons::map_weather_description_to_code;
use chrono::{NaiveDateTime, TimeZone, Local};

const API_KEY: &str = "2a33d8b44aa8d93d07feac453b4a79aa";

const GREEN: &str = "\x1b[32m";

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherData,
    weather: Vec<WeatherDescription>,
    sys: SysData,
    wind: WindData,
    visibility: f32,
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
struct WeatherDescription {
    description: String,
}

#[derive(Deserialize)]
struct SysData {
    sunrise: i64,
    sunset: i64,
}


#[derive(Deserialize)]
struct WindData {
    deg: u16,
    speed: f32,
}



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


//Function to print the weather of the 4 upcoming days
pub fn weather_forecast() -> Result<(), Box<dyn std::error::Error>> {
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

    let url = format!("http://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units={}", city_name, API_KEY,unit_type);

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

    for forecast_data in forecast.list.iter().step_by(8).take(4) {        let weather_description = match forecast_data.weather.get(0) {
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

    let date_time = &forecast_data.dt_txt;
    let date = &date_time.split(' ').collect::<Vec<&str>>()[0];
    
    let formatted_temp = format!("{}°{}", forecast_data.main.temp, unit_value);
    let formatted_humidity = format!("{}%", forecast_data.main.humidity);
    
    let temp_box = [
        "╔══════════════════╗",
        &format!("║{: ^18}║", date),
        &format!("║{: ^18}║", weather_status.icon()), 
        &format!("║{: ^18}║", formatted_temp), 
        &format!("║{: ^18}║", formatted_humidity),
        "╚══════════════════╝",
    ].join("\n");
    
    println!("{}", temp_box);
    
    
    }

    Ok(())
}


fn fetch_weather_data() -> Result<WeatherResponse, Box<dyn std::error::Error>> {

    let city_name = read_city_name()?;
    let unit_value = read_unit()?;
    let unit_type = if unit_value == "C" {
        "metric"
    } else {
        "imperial"
    };

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        city_name, API_KEY, unit_type
    );

    let response: serde_json::Value = reqwest::blocking::get(&url)?.json()?;
    if response["cod"] != 200 {
        return Err(format!("Error: {}", response["message"]).into());
    }
    serde_json::from_value(response).map_err(Into::into)
}


//Function to fetch the weather now
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

    let formatted_temp = format!(" {}°{}", weather.main.temp, read_unit()?);
    let formatted_humidity = format!(" {}%", weather.main.humidity);
    println!("•{}", read_city_name()?);
    let temp_box = [
        "╔══════════════════╗",
        &format!("║{: ^18}║", weather_status.icon()), 
        &format!("║{: ^18}║", weather.weather[0].description),
        &format!("║{: ^18}║", formatted_temp), 
        &format!("║{: ^18}║", formatted_humidity),
        "╚══════════════════╝",
    ].join("\n");
    
    println!("{}", temp_box);
    
    Ok(())
}


//Function to fetch more info about the weather now
pub fn weather_details() -> Result<(), Box<dyn std::error::Error>> {

    let weather = fetch_weather_data()?;
    println!("{}{}•City: {}{}",GREEN,BOLD,RESET, read_city_name()?);
    println!("{}{}•Temperature: {}{}°{}",GREEN,BOLD,RESET, weather.main.temp, read_unit()?);
    println!("{}{}•Feels Like: {}{}°{}",GREEN,BOLD,RESET, weather.main.feels_like, read_unit()?);
    println!("{}{}•Weather Description: {}{}",GREEN,BOLD,RESET, weather.weather[0].description);
    println!("{}{}•Minimum Temperature: {}{}°{}",GREEN,BOLD,RESET, weather.main.temp_min, read_unit()?);
    println!("{}{}•Maximum Temperature: {}{}°{}",GREEN,BOLD,RESET, weather.main.temp_max, read_unit()?);
    println!("{}{}•Humidity: {}{}%",GREEN,BOLD,RESET, weather.main.humidity);
    println!("{}{}•Pressure: {}{} hPa",GREEN,BOLD,RESET, weather.main.pressure);
    println!("{}{}•Sunrise: {}{}",GREEN,BOLD,RESET, unix_time_to_datetime(weather.sys.sunrise));
    println!("{}{}•Sunset: {}{}",GREEN,BOLD,RESET, unix_time_to_datetime(weather.sys.sunset));
    println!("{}{}•Visibily: {}{}m",GREEN,BOLD,RESET, weather.visibility);
    println!("{}{}•Wind Degree: {}{}°",GREEN,BOLD,RESET, weather.wind.deg);
    let speed_value = if read_unit()? == "C" { "m/s" } else { "miles/h" };
    println!("{}{}•Wind Speed: {}{} {}",GREEN,BOLD,RESET, weather.wind.speed,speed_value);
    Ok(())
}


//Function to convert unix time to local datetime
fn unix_time_to_datetime(timestamp: i64) -> String {
    let naive_datetime = DateTime::from_timestamp(timestamp, 0);

    let local_datetime = Local.from_utc_datetime(&naive_datetime);

    let formatted_time = local_datetime.format("%H:%M:%S").to_string();

    formatted_time
}