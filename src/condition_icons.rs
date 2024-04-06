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

// Mapping from weather description to weather code
pub fn map_weather_description_to_code(description: &str) -> Option<u16> {
    match description {
        "thunderstorm with light rain" => Some(200),
        "thunderstorm with rain" => Some(201),
        "thunderstorm with heavy rain" => Some(202),
        "light thunderstorm" => Some(210),
        "thunderstorm" => Some(211),
        "heavy thunderstorm" => Some(212),
        "ragged thunderstorm" => Some(221),
        "thunderstorm with light drizzle" => Some(230),
        "thunderstorm with drizzle" => Some(231),
        "thunderstorm with heavy drizzle" => Some(232),
        "light intensity drizzle" => Some(300),
        "drizzle" => Some(301),
        "heavy intensity drizzle" => Some(302),
        "light intensity drizzle rain" => Some(310),
        "drizzle rain" => Some(311),
        "heavy intensity drizzle rain" => Some(312),
        "shower rain and drizzle" => Some(313),
        "heavy shower rain and drizzle" => Some(314),
        "shower drizzle" => Some(321),
        "light rain" => Some(500),
        "moderate rain" => Some(501),
        "heavy intensity rain" => Some(502),
        "very heavy rain" => Some(503),
        "extreme rain" => Some(504),
        "freezing rain" => Some(511),
        "light intensity shower rain" => Some(520),
        "shower rain" => Some(521),
        "heavy intensity shower rain" => Some(522),
        "ragged shower rain" => Some(531),
        "light snow" => Some(600),
        "snow" => Some(601),
        "heavy snow" => Some(602),
        "sleet" => Some(611),
        "light shower sleet" => Some(612),
        "shower sleet" => Some(613),
        "light rain and snow" => Some(615),
        "rain and snow" => Some(616),
        "light shower snow" => Some(620),
        "shower snow" => Some(621),
        "heavy shower snow" => Some(622),
        "mist" => Some(701),
        "smoke" => Some(711),
        "haze" => Some(721),
        "sand/dust whirls" => Some(731),
        "fog" => Some(741),
        "sand" => Some(751),
        "dust" => Some(761),
        "volcanic ash" => Some(762),
        "squalls" => Some(771),
        "tornado" => Some(781),
        "clear sky" => Some(800),
        "few clouds" => Some(801),
        "scattered clouds" => Some(802),
        "broken clouds" => Some(803),
        "overcast clouds" => Some(804),
        _ => None,
    }
}