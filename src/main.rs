use clap::Parser;
use serde::Deserialize;

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
    api_key: String,
}

pub fn name_pass() -> (String, String) {
    let args = Cli::parse();
    (args.city, args.api_key)
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yes = name_pass();
    let api = yes.1;

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
        println!("City: {}", yes.0);

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
