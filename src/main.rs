use crate::connectdb::Database;
use crate::temperature::save_temperature_data;
mod connectdb;
mod request;
mod surrealmodel;
mod weathermodel;
mod temperature;
mod wind;
mod cities;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let db = connectdb::Database::new().await?;
    
    let weather_response = request::fetch_weather_for_all_cities_typed(db.db).await;

    match weather_response {
        Some(weather_data) => {
            /*println!(
                "Weather response received for {} cities",
                weather_data.len()
            );*/

            // Reuse the existing database connection
            // Save full weather data to weather table
            /*for city_weather in &weather_data {
                db.save_weather_data(city_weather).await?;
            }*/

            /*for city_weather in &weather_data {
                db.save_sunset_data(city_weather).await?;
            }
            // Save temperature data to temperature table
            for city_weather in &weather_data {
                temperature::save_temperature_data(db.db, city_weather).await?;
            }
            // Save wind data
            for city_weather in &weather_data {
                wind::save_wind_data(db.db, city_weather).await?;
            }*/
        }
        None => {
            println!("No weather data received from API");
        }
    }

    Ok(())
}
