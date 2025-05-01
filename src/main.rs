use dotenv::dotenv;
use reqwest::Url;
use std::env;
use std::error::Error;

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

    let body = reqwest::blocking::get(endpoint)?.text()?;

    println!("{body}");

    Ok(())
}
