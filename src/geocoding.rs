use serde::Deserialize;

use crate::get_api_key;
use crate::name_pass;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

pub fn location_name() -> String {
    let cli = name_pass();
    let api = get_api_key(cli.1);

    format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
        cli.0, api
    )
}
