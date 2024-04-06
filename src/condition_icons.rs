// condition_icons.rs

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
    pub fn from_weather_code(code: u16) -> Option<WeatherStatus> {
        match code {
            200..=202 | 210..=212 | 221 | 230..=232 => Some(WeatherStatus::Thunderstorm),
            300..=302 | 310..=314 | 321 => Some(WeatherStatus::ShowerRain),
            500 | 520 | 521 => Some(WeatherStatus::Rain),
            501..=504 => Some(WeatherStatus::Rain),
            511 => Some(WeatherStatus::Snow),
            600..=601 | 611..=612 | 615..=616 | 620..=622 => Some(WeatherStatus::Snow),
            701..=781 => Some(WeatherStatus::Mist),
            800 => Some(WeatherStatus::ClearSky),
            801 => Some(WeatherStatus::FewClouds),
            802 => Some(WeatherStatus::ScatteredClouds),
            803..=804 => Some(WeatherStatus::BrokenClouds),
            _ => None,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WeatherStatus::ClearSky => "",
            WeatherStatus::FewClouds => "",
            WeatherStatus::ScatteredClouds => "",
            WeatherStatus::BrokenClouds => "",
            WeatherStatus::ShowerRain => "",
            WeatherStatus::Rain => "",
            WeatherStatus::Thunderstorm => "",
            WeatherStatus::Snow => "",
            WeatherStatus::Mist => "󰖑",
        }
    }
}
