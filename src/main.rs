mod cities;
mod connectdb;
mod request;
mod surrealmodel;
mod weathermodel;
mod temperature;
mod wind;
mod mainweather;
mod sunset;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let db = connectdb::Database::new().await?;
    
    let weather_response = request::fetch_weather_for_french_cities(db.db).await;

    match weather_response {
        Some(weather_data) => {
            println!(
                "Weather response received for {} cities",
                weather_data.len()
            );

            // Reuse the existing database connection
            // Save full weather data to weather table
            for city_weather in &weather_data {
                mainweather::save_main_weather(db.db, city_weather).await?;
            }

            for city_weather in &weather_data {
                sunset::save_sunset_data(db.db, city_weather).await?;
            }
            // Save temperature data to temperature table
            /*for city_weather in &weather_data {
                temperature::save_temperature_data(db.db, city_weather).await?;
            }
            // Save wind data
            for city_weather in &weather_data {
                wind::save_wind_data_simple(city_weather).await?;
            }*/
        }
        None => {
            println!("No weather data received from API");
        }
    }

    Ok(())
}