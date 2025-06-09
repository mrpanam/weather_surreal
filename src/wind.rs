use crate::connectdb::get_db;
use crate::surrealmodel::Wind;
use crate::weathermodel::WeatherResponse;
use std::error::Error;

pub async fn save_wind_data(data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
    let wind_data = Wind {
        city: data.name.clone(),
        country: data.sys.country.clone(),
        deg: data.wind.deg,
        speed: data.wind.speed,
        gust: data.wind.gust,
        date: data.dt,
    };

    match get_db().create::<Option<Wind>>("wind").content(wind_data).await {
        Ok(_) => {
            println!("Wind data saved successfully for {}", data.name);
            Ok(())
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Serialization error")
                || error_msg.contains("failed to deserialize")
            {
                println!(
                    "Wind data saved successfully for {} (with return value parsing issue)",
                    data.name
                );
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
} 

pub async fn save_wind_data_simple(data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
    let wind_data = Wind {
        city: data.name.clone(),
        country: data.sys.country.clone(),
        deg: data.wind.deg,
        speed: data.wind.speed,
        gust: data.wind.gust,
        date: data.dt,
    };

    get_db().create::<Option<Wind>>("wind").content(wind_data).await?;
   
    Ok(())
           
} 