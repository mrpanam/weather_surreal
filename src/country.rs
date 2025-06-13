use serde::{Deserialize, Serialize};
use crate::surrealmodel::Country;
use surrealdb::RecordId;
use std::error::Error;
use crate::connectdb::get_db;


impl Country {
    /// Create a new country
    pub async fn create(name: String, code: String) -> Result<Self, Box<dyn Error>> {
        let country = Country {
            id: None,
            name: name.clone(),
            code: code.clone(),
        };
        
        let created: Option<Country> = get_db()
            .create("country")
            .content(country)
            .await?;
            
        match created {
            Some(country) => Ok(country),
            None => Err("Failed to create country".into())
        }
    }
    
    /// Get a country by its ID
    pub async fn get_by_id(id: &str) -> Result<Option<Self>, Box<dyn Error>> {
        let country: Option<Country> = get_db().select(("country", id)).await?;
        Ok(country)
    }
    
    /// Get a country by its code
    pub async fn get_by_code(code: String) -> Result<Option<Self>, Box<dyn Error>> {
        let mut result = get_db()
            .query("SELECT * FROM country WHERE code = $code LIMIT 1")
            .bind(("code", code))
            .await?;
            
        let countries: Vec<Country> = result.take(0)?;
        Ok(countries.into_iter().next())
    }
    
    /// Get or create a country
    pub async fn get_or_create(name: String, code: String) -> Result<Self, Box<dyn Error>> {
        if let Some(country) = Self::get_by_code(code.clone()).await? {
            Ok(country)
        } else {
            Self::create(name, code).await
        }
    }
    
    /// Get all countries
    pub async fn get_all() -> Result<Vec<Self>, Box<dyn Error>> {
        let countries: Vec<Country> = get_db().select("country").await?;
        println!("Found {} countries", countries.len());
        Ok(countries)
    }
}
