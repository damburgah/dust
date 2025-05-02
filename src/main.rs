use dotenv::dotenv;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: u8,
    timezone: String,
    timezone_abbreviation: String,
    // elevation: u8,
    elevation: f64,
    current_units: CurrentUnits,
    current: CurrentData,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentUnits {
    time: String,
    interval: String,
    us_aqi: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentData {
    time: String,
    interval: u16,
    us_aqi: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut endpoint = Url::parse("https://air-quality-api.open-meteo.com/v1/air-quality")?;

    let current = "us_aqi";
    let latitude = env::var("LATITUDE")?;
    let longitude = env::var("LONGITUDE")?;

    endpoint.query_pairs_mut()
        .append_pair("current", current)
        .append_pair("latitude", latitude.as_str())
        .append_pair("longitude", longitude.as_str());

    let response: ApiResponse = reqwest::blocking::get(endpoint)?.json()?;

    // println!("{:#?}", response);
    println!("Tay Ho AQI (satellite): {}", response.current.us_aqi);

    Ok(())
}
