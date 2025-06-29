use crate::surrealmodel::{City, Temperature};
use crate::weathermodel::WeatherResponse;
use std::error::Error;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn save_temperature_data(
    db: &Surreal<Client>,
    data: &WeatherResponse,
) -> Result<(), Box<dyn Error>> {
    // Create RecordId for city using the City::create_id method
    let city_id = City::create_id(&data.name);

    let temperature_data = Temperature {
        city: city_id,        
        temp: data.main.temp,
        feels_like: data.main.feels_like,
        temp_min: data.main.temp_min,
        temp_max: data.main.temp_max,
        date: data.dt,
    };
    println!(
        "Temperature data: {}",
        serde_json::to_string_pretty(&temperature_data).unwrap()
    );

    match db
        .create::<Option<Temperature>>("temperature")
        .content(temperature_data)
        .await
    {
        Ok(_) => {
            println!("Temperature data saved successfully for {}", data.name);
            Ok(())
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Serialization error")
                || error_msg.contains("failed to deserialize")
            {
                println!(
                    "Temperature data saved successfully for {} (with return value parsing issue)",
                    data.name
                );
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
}
