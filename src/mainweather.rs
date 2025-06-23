use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Error as SurrealError;
use crate::surrealmodel::MainWeather;
use crate::surrealmodel::City;
use crate::weathermodel::WeatherResponse;

/// Saves main weather data to the SurrealDB database
/// 
/// # Arguments
/// * `db` - A reference to the SurrealDB client
/// * `weather` - The MainWeather data to be saved
/// 
/// # Returns
/// * `Result<MainWeather, Db>` - Returns the saved MainWeather on success, or a database error
pub async fn save_main_weather(
    db: &Surreal<Client>,
    data: &WeatherResponse
) -> Result<MainWeather, SurrealError> {
    let city_id = City::create_id(&data.name);

    let main_weather = MainWeather {
        id: None,
        city: city_id,
        temp: data.main.temp,
        feels_like: data.main.feels_like,
        temp_min: data.main.temp_min,
        temp_max: data.main.temp_max,
        pressure: data.main.pressure,
        humidity: data.main.humidity,
        sea_level: data.main.sea_level,
        grnd_level: data.main.grnd_level,
        date: data.dt,
    };
    // Save the weather data to the database and let SurrealDB handle ID generation
    let saved: Option<MainWeather> = db
        .create("main_weather")
        .content(main_weather)
        .await?;

    saved.ok_or_else(|| SurrealError::Api(surrealdb::error::Api::InternalError("Failed to save main weather data".to_string())))
}


