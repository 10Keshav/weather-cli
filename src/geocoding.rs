use serde::Deserialize;

use crate::name_pass;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

pub fn location_name() -> String {
    let yes = name_pass();

    format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
        yes.0, yes.1
    )
}
