use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Flight {
    pub flight_id: u32,
    pub pilot_id: u32,
    pub aircraft_id: u32,
    pub flight_plan: String,
    pub departure_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherRequest {
    pub latitude: f64,
    pub longitude: f64,
}