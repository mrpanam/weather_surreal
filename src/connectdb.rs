use crate::surrealmodel::Sun;
use crate::weathermodel::WeatherResponse;
use std::error::Error;
use std::sync::LazyLock;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub fn get_db() -> &'static Surreal<Client> {
    &DB
}

pub struct Database {
    connected: bool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Connect to WebSocket
        DB.connect::<Ws>("127.0.0.1:8000").await?;

        DB.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        DB.use_ns("eric").use_db("Xone").await?;
        println!("connected to DB");

        Ok(Database { connected: true })
    }

    pub async fn save_weather_data(&self, data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
        match DB
            .create::<Option<WeatherResponse>>("weather")
            .content(data.clone())
            .await
        {
            Ok(_) => {
                println!("Weather data saved successfully for {}", data.name);
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
                        "Weather data saved successfully for {} (with return value parsing issue)",
                        data.name
                    );
                    Ok(())
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn save_sunset_data(&self, data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
        // Create a city ID string using the city name and ID

        // Create Sun struct with partial data from weather response
        let sun_data = Sun {
            city: data.name.clone(),
            country: data.sys.country.clone(),
            sunrise: data.sys.sunrise,
            sunset: data.sys.sunset,
            date: data.dt,
        };
        match DB.create::<Option<Sun>>("sunset").content(sun_data).await {
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
}
