use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sun {
    pub city: String,    
    pub sunrise: u64,
    pub sunset: u64,
    pub date: u64,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct City {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    #[serde(skip_serializing)]
    pub country: RecordId,
    pub name: String,
    pub lon: f32,
    pub lat: f32,
}

impl City {
    /// Creates a RecordId from a city name by converting it to lowercase and replacing spaces with underscores
    pub fn create_id(name: &str) -> RecordId {
        let id_str = name.to_lowercase()
            .replace(' ', "_")
            .replace("-", "_")
            .replace("è", "e")
            .replace("é", "e");
        RecordId::from(("city", id_str.as_str()))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MainWeather {
    pub id: Option<RecordId>,
    pub city: RecordId,   
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
    pub sea_level: u32,
    pub grnd_level: u32,
    pub date: u64,
}



#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: Option<RecordId>,
    pub name: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    pub city: RecordId,
    //pub country: RecordId,
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub date: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub city: String,
    pub country: String,
    pub deg: u32,
    pub speed: f64,
    pub gust: Option<f64>,
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
            "City: {},\nDate: {}\nSunrise: {}\nSunset: {}",
            self.city,            
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
            "City: {},\nDate: {}\nTemperature: {:.1}°C (feels like {:.1}°C)\nMin: {:.1}°C, Max: {:.1}°C",
            self.city,
            //self.country,
            self.format_date(),
            self.temp,
            self.feels_like,
            self.temp_min,
            self.temp_max
        )
    }
}
