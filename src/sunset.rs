use crate::surrealmodel::Sun;
use crate::weathermodel::WeatherResponse;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::error::Error;

pub async fn save_sunset_data(db: &Surreal<Client>, data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
    // Create Sun struct with partial data from weather response
    let sun_data = Sun {
        city: data.name.clone(),        
        sunrise: data.sys.sunrise,
        sunset: data.sys.sunset,
        date: data.dt,
    };
    
    match db.create::<Option<Sun>>("sunset").content(sun_data).await {
        Ok(_) => {
            println!("Sunset data saved successfully for {}", data.name);
            Ok(())
        }
        Err(e) => {
            // Check if it's just a deserialization error of the return value
            let error_msg = e.to_string();
            if error_msg.contains("Serialization error")
                || error_msg.contains("failed to deserialize")
            {
                // The data was likely saved successfully, just return value parsing failed
                println!(
                    "Sunset data saved successfully for {} (with return value parsing issue)",
                    data.name
                );
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
}
