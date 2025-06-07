use crate::connectdb::Database;
mod cities;
mod connectdb;
mod request;
mod surrealmodel;
mod weathermodel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather_response = request::fetch_weather_for_all_cities_typed().await;

    match weather_response {
        Some(weather_data) => {
            println!(
                "Weather response received for {} cities",
                weather_data.len()
            );

            let db = Database::new().await?;
            // Save full weather data to weather table
            /*for city_weather in &weather_data {
                db.save_weather_data(city_weather).await?;
            }*/

            for city_weather in &weather_data {
                db.save_sunset_data(city_weather).await?;
            }
            // Save temperature data to temperature table

            for city_weather in &weather_data {
                db.save_temperature_data(city_weather).await?;
            }
        }
        None => {
            println!("No weather data received from API");
        }
    }

    Ok(())
}
