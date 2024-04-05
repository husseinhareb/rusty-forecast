#[derive(Debug, PartialEq)]
pub enum WeatherStatus {
    ClearSky,
    FewClouds,
    ScatteredClouds,
    BrokenClouds,
    ShowerRain,
    Rain,
    Thunderstorm,
    Snow,
    Mist,
}

impl WeatherStatus {
   pub fn icon(&self) -> &'static str {
        match self {
            WeatherStatus::ClearSky => " ",
            WeatherStatus::FewClouds => " ",
            WeatherStatus::ScatteredClouds => "",
            WeatherStatus::BrokenClouds => "",
            WeatherStatus::ShowerRain => " ",
            WeatherStatus::Rain => "",
            WeatherStatus::Thunderstorm => "",
            WeatherStatus::Snow => "",
            WeatherStatus::Mist => "󰖑",
        }
    }
}

