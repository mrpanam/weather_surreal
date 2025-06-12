use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherResponse {
    pub base: String,
    pub clouds: Clouds,
    pub cod: u32,
    pub coord: Coordinates,
    pub dt: u64,
    pub id: u32,
    pub main: Main,
    pub name: String,
    pub sys: Sys,
    pub timezone: u32,
    pub visibility: u32,
    pub weather: Vec<Weather>,
    pub wind: Wind,    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<Rain>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Main {
    pub feels_like: f64,
    pub grnd_level: u32,
    pub humidity: u32,
    pub pressure: u32,
    pub sea_level: u32,
    pub temp: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sys {
    pub country: String,
    pub id: Option<u32>,
    pub sunrise: u64,
    pub sunset: u64,
    #[serde(rename = "type")]
    pub type_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weather {
    pub description: String,
    pub icon: String,
    pub id: u32,
    pub main: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wind {
    pub deg: u32,
    pub speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gust: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub one_hour: Option<f64>,
    #[serde(rename = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hours: Option<f64>,
}

impl WeatherResponse {
    pub fn format_datetime(&self) -> String {
        DateTime::<Utc>::from_timestamp(self.dt as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| format!("Invalid timestamp: {}", self.dt))
    }

    pub fn format_summary_with_datetime(&self) -> String {
        format!(
            "--- Weather for {} ---\nTimestamp: {} ({})\nTemperature: {:.2}C\nHumidity: {}%\nCondition: {}\n",
            self.name,
            self.format_datetime(),
            self.dt,
            self.main.temp,
            self.main.humidity,
            self.weather.first().map_or("Unknown", |w| &w.description)
        )
    }
}
