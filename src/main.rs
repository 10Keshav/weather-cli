use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    desc: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "fetch cur weather for a given city")]
struct Args {
    #[arg(short, long)]
    city: String,

    #[arg(short, long)]
    api_key: String,
}

fn main() {
    println!("Hello, world!");
}
