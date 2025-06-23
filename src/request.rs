use crate::cities;
use crate::weathermodel::WeatherResponse;
use reqwest::Client;
use surrealdb::{Surreal, engine::remote::ws::Client as DbClient};

pub async fn fetch_weather_for_french_cities(db: &Surreal<DbClient>) -> Option<Vec<WeatherResponse>> {
    let cities = match cities::get_cities_fr(db).await {
        Ok(cities) => cities,
        Err(e) => {
            eprintln!("Failed to fetch cities: {}", e);
            return None;
        }
    };
    
    let client = Client::new();
    let mut weather_responses = Vec::new();

    println!("Calling OpenWeatherMap API for each city...");

    for city in cities {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid=3e706d704abbc847d0b5ebe9c6969d0d",
            city.lat, city.lon
        );

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<WeatherResponse>().await {
                        Ok(weather_data) => {
                            println!("Timestamp: {}", weather_data.format_datetime());
                            println!("{}", serde_json::to_string_pretty(&weather_data).unwrap());
                            weather_responses.push(weather_data);
                        }
                        Err(e) => {
                            eprintln!("Error parsing JSON for {}: {}", city.name, e);
                        }
                    }
                } else {
                    eprintln!(
                        "Error fetching weather for {}: HTTP status {}",
                        city.name,
                        response.status()
                    );
                }
            }
            Err(e) => {
                eprintln!("Request failed for {}: {}", city.name, e);
            }
        }
    }

    if weather_responses.is_empty() {
        None
    } else {
        Some(weather_responses)
    }
}
