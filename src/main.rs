use clap::Parser;
use serde::Deserialize;
use std::fs;

mod geocoding;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<WeatherCondition>,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u32,
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    main: String,
    description: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "fetch cur weather for a given city")]
struct Cli {
    #[arg(short = 'c', long = "city")]
    city: String,

    #[arg(global = false, short = 'a', long = "api-key")]
    api_key: Option<String>,
}

pub fn get_api_key(key_cli: Option<String>) -> String {
    if let Some(mut path) = dirs::config_dir() {
        path.push("weather-cli");
        path.push("api_key.txt");
        if let Ok(key) = fs::read_to_string("api_key.txt") {
            let trimmed = key.trim().to_string();
            if !trimmed.is_empty() {
                return trimmed;
            }
        }
    }

    if let Ok(key) = fs::read_to_string("api_key.txt") {
        let trimmed = key.trim().to_string();
        if !trimmed.is_empty() {
            return trimmed;
        }
    }

    if let Some(key) = key_cli {
        return key;
    }

    eprintln!("Err: No API key provided");
    eprintln!("Please pass -a <KEY> or put your key in 'api_key.txt'.");
    std::process::exit(1);
}

fn name_pass() -> (String, Option<String>) {
    let args = Cli::parse();
    (args.city, args.api_key)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let city = args.city;
    let api = get_api_key(args.api_key);

    let call = geocoding::location_name();
    let resp: Vec<geocoding::Location> = reqwest::get(call)
        .await?
        .json::<Vec<geocoding::Location>>()
        .await?;
    if let Some(loc) = resp.first() {
        let call = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
            loc.lat, loc.lon, api
        );
        println!("City: {}", city);

        let w_resp: WeatherResponse = reqwest::get(call).await?.json::<WeatherResponse>().await?;

        println!("Temp: {}°C", w_resp.main.temp);
        println!("Humidity: {}%", w_resp.main.humidity);
        if let Some(cond) = w_resp.weather.first() {
            println!("Condition: {}\n{}", cond.main, cond.description);
        }
    } else {
        println!("no such city");
    }
    Ok(())
}
