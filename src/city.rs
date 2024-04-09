use reqwest::Error;

#[derive(Debug, serde::Deserialize)]
struct IpInfo {
    city: Option<String>,
}

pub fn default_city() -> Result<Option<String>, Error> {
    let response: IpInfo = reqwest::blocking::get("https://ipinfo.io/json")?.json()?;
    Ok(response.city)
}
