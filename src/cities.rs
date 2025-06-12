use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Deserialize, Serialize, Debug)]
pub struct City {
    pub id: Option<RecordId>,
    pub country: RecordId,
    pub name: String,
    pub lon: f32,
    pub lat: f32,
}


pub async fn get_cities(db: &Surreal<Client>) -> Result<Vec<City>, Box<dyn std::error::Error>> {
    println!("Fetching cities from database...");
    let cities: Vec<City> = db.select("city").await?;
    println!("Found {} cities", cities.len());
    for city in &cities {
        println!("City: {}, Country: {}", city.name, city.country);
    }
    Ok(cities)
}

pub async fn get_cities_france(db: &Surreal<Client>) -> Result<Vec<City>, Box<dyn std::error::Error>> {
    // Query directly for French cities using SurrealQL
    let mut result = db
        .query("SELECT * FROM city WHERE country = country:france")
        .await?;
    
    // Take the first result set and convert it to Vec<City>
    let france_cities: Vec<City> = result.take(0)?;
    
    Ok(france_cities)
}
