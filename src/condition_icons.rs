#[derive(Debug, PartialEq)]
pub enum WeatherStatus {
    clear_sky,
    few_clouds,
    scattered_clouds,
    broken_clouds,
    shower_rain,
    rain,
    thunderstorm,
    snow,
    mist,
}

impl WeatherStatus {
   pub fn icon(&self) -> &'static str {
        match self {
            WeatherStatus::clear_sky => " ",
            WeatherStatus::few_clouds => " ",
            WeatherStatus::scattered_clouds => "",
            WeatherStatus::broken_clouds => "",
            WeatherStatus::shower_rain => " ",
            WeatherStatus::rain => "",
            WeatherStatus::thunderstorm => "",
            WeatherStatus::snow => "",
            WeatherStatus::mist => "󰖑",
        }
    }
}

