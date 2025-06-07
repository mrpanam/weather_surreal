use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sun {
    pub city: String,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
    pub date: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    pub city: String,
    pub country: String,
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub date: u64,
}

impl Sun {
    pub fn format_sunrise(&self) -> String {
        DateTime::<Utc>::from_timestamp(self.sunrise as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| format!("Invalid timestamp: {}", self.sunrise))
    }

    pub fn format_sunset(&self) -> String {
        DateTime::<Utc>::from_timestamp(self.sunset as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| format!("Invalid timestamp: {}", self.sunset))
    }

    pub fn format_date(&self) -> String {
        DateTime::<Utc>::from_timestamp(self.date as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| format!("Invalid timestamp: {}", self.date))
    }

    pub fn format_summary(&self) -> String {
        format!(
            "City: {}, {}\nDate: {}\nSunrise: {}\nSunset: {}",
            self.city,
            self.country,
            self.format_date(),
            self.format_sunrise(),
            self.format_sunset()
        )
    }
}

impl Temperature {
    pub fn format_date(&self) -> String {
        DateTime::<Utc>::from_timestamp(self.date as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| format!("Invalid timestamp: {}", self.date))
    }

    pub fn format_summary(&self) -> String {
        format!(
            "City: {}, {}\nDate: {}\nTemperature: {:.1}°C (feels like {:.1}°C)\nMin: {:.1}°C, Max: {:.1}°C",
            self.city,
            self.country,
            self.format_date(),
            self.temp,
            self.feels_like,
            self.temp_min,
            self.temp_max
        )
    }
}