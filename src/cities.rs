use crate::surrealmodel::City;
use axum::Json;
use surrealdb::{Surreal, engine::remote::ws::Client};


pub async fn get_cities(db: &Surreal<Client>) -> Result<Vec<City>, Box<dyn std::error::Error>> {
    match db.select::<Vec<City>>("city").await {
        Ok(cities) => Ok(cities),
        Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>)
    }
}



pub async fn get_cities_fr(db: &Surreal<Client>) -> Result<Vec<City>, Box<dyn std::error::Error>> {
    let mut response = db.query("SELECT * FROM city WHERE country=country:FR AND weather IS NOT NULL").await?;
    let cities: Vec<City> = response.take(0)?;
    Ok(cities)
}
