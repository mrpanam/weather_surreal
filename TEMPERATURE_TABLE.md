# Temperature Table Implementation

This document explains the temperature table functionality that saves temperature data from the OpenWeatherMap API to SurrealDB.

## Overview

The temperature table stores temperature information for cities, extracted from the full weather response. This demonstrates how to save only temperature-related fields from a larger API response to a dedicated table.

## Data Structure

The `Temperature` struct represents the data stored in the `temperature` table:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    pub city: String,        // City name
    pub country: String,     // Country code (e.g., "US", "JP")
    pub temp: f64,          // Current temperature in Celsius
    pub feels_like: f64,    // "Feels like" temperature in Celsius
    pub temp_min: f64,      // Minimum temperature in Celsius
    pub temp_max: f64,      // Maximum temperature in Celsius
    pub date: u64,          // Unix timestamp for the date
}
```

## Key Features

### 1. Temperature Data Extraction
The system extracts temperature data from the full weather response:
- `temp`, `feels_like`, `temp_min`, `temp_max` from `weather_response.main`
- `city` from `weather_response.name`
- `country` from `weather_response.sys.country`
- `date` from `weather_response.dt`

### 2. Database Operations
- **Save**: `save_temperature_data(&self, data: &WeatherResponse)`
- **Retrieve All**: `get_all_temperature_data(&self)`
- **Retrieve by City**: `get_temperature_data(&self, city_name: &str)`
- **Retrieve by Range**: `get_temperature_by_range(&self, min_temp: f64, max_temp: f64)`
- **Clear Table**: `clear_temperature_table(&self)`

### 3. Formatting Utilities
The `Temperature` struct includes methods for human-readable formatting:
- `format_date()` - Formatted date
- `format_summary()` - Complete temperature summary

## Usage

### Running the Application

```bash
# Run the main application
cargo run
```

### Example Output

```
Weather response received for 8 cities
Connected to DB for temperature service
Temperature data saved successfully for London
Temperature data saved successfully for Tokyo
Temperature data saved successfully for Paris
...

=== Retrieving Temperature Data ===
Retrieved 8 temperature records

City: London, GB
Date: 2025-06-06
Temperature: 18.5°C (feels like 17.8°C)
Min: 16.2°C, Max: 21.3°C

City: Tokyo, JP
Date: 2025-06-06
Temperature: 24.1°C (feels like 26.3°C)
Min: 22.8°C, Max: 25.7°C
...

=== Temperatures between 15°C and 25°C ===
Retrieved 5 temperature records in range 15.0°C to 25.0°C
```

## Database Schema

The `temperature` table in SurrealDB contains records like:

```json
{
    "id": "temperature:⟨record_id⟩",
    "city": "London",
    "country": "GB",
    "temp": 18.5,
    "feels_like": 17.8,
    "temp_min": 16.2,
    "temp_max": 21.3,
    "date": 1749214213
}
```

## SurrealDB Queries

You can query the temperature table directly using SurrealQL:

```sql
-- Get all temperature records
SELECT * FROM temperature;

-- Get temperature data for a specific city
SELECT * FROM temperature WHERE city CONTAINS 'London';

-- Get temperatures in a specific range
SELECT * FROM temperature WHERE temp >= 15 AND temp <= 25;

-- Get cities with high temperatures (above 30°C)
SELECT * FROM temperature WHERE temp > 30;

-- Get cities where it feels hotter than actual temperature
SELECT * FROM temperature WHERE feels_like > temp;

-- Get temperature records for a specific date range
SELECT * FROM temperature WHERE date > 1749214200 AND date < 1749300600;

-- Average temperature by country
SELECT country, math::mean(temp) AS avg_temp FROM temperature GROUP BY country;
```

## Implementation Details

### Single Database Connection Architecture

The temperature functionality is now integrated into the main `Database` struct in `connectdb.rs`, providing a single, efficient connection for all weather data operations (temperature, sunset, etc.).

### Data Extraction

```rust
impl Database {
    pub async fn save_temperature_data(&self, data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
        // Create Temperature struct with data from weather response
        let temperature_data = Temperature {
            city: data.name.clone(),
            country: data.sys.country.clone(),
            temp: data.main.temp,
            feels_like: data.main.feels_like,
            temp_min: data.main.temp_min,
            temp_max: data.main.temp_max,
            date: data.dt,
        };

        // Save to database using shared connection
        DB.create::<Option<Temperature>>("temperature").content(temperature_data).await
    }
}
```

### Data Retrieval Examples

```rust
// Initialize single database connection
let db = Database::new().await?;

// Get all temperature data
let all_temp_data = db.get_all_temperature_data().await?;

// Get temperature data for a specific city
let city_temp_data = db.get_temperature_data("London").await?;

// Get temperatures in a specific range (15°C to 25°C)
let temp_range_data = db.get_temperature_by_range(15.0, 25.0).await?;

// Can also access sunset data with same connection
let sunset_data = db.get_all_sunset_data().await?;
```

## Use Cases

1. **Weather Monitoring**: Track temperature changes over time
2. **Climate Analysis**: Compare temperatures across different cities
3. **Temperature Alerts**: Find cities with extreme temperatures
4. **Comfort Index**: Analyze "feels like" vs actual temperature differences
5. **Regional Comparison**: Group and compare temperatures by country

## Benefits

1. **Single Connection**: Efficient resource usage with one database connection
2. **Focused Data Storage**: Only store temperature-related data
3. **Efficient Queries**: Faster queries on temperature-specific fields
4. **Range Queries**: Easy to find temperatures within specific ranges
5. **Statistical Analysis**: Support for aggregation and grouping operations
6. **Flexible Retrieval**: Multiple ways to query temperature data
7. **Unified API**: Access all weather data (temperature, sunset, etc.) through one interface

## Error Handling

The system includes robust error handling:
- Database connection errors
- Serialization/deserialization errors
- Query execution errors
- Range validation for temperature queries

All database operations return `Result<T, Box<dyn Error>>` for proper error handling.

## Database API (Single Connection)

### Database Methods for Temperature

```rust
// Initialize the single database connection
let db = Database::new().await?;

// Save temperature data from weather response
db.save_temperature_data(&weather_response).await?;

// Get all temperature records
let all_temps = db.get_all_temperature_data().await?;

// Get temperature records for a specific city
let city_temps = db.get_temperature_data("Paris").await?;

// Get temperatures within a range
let range_temps = db.get_temperature_by_range(20.0, 30.0).await?;

// Clear all temperature records (use with caution)
db.clear_temperature_table().await?;

// Bonus: Access other weather data with same connection
let sunset_data = db.get_all_sunset_data().await?;
```

## Maintenance

If you need to clear the temperature table for any reason:

```rust
// Clear all temperature records (use with caution)
db.clear_temperature_table().await?;
```

This is useful for schema changes or data cleanup operations.

## Future Enhancements

Potential improvements to consider:

1. **Historical Tracking**: Store multiple temperature readings per city over time
2. **Temperature Conversion**: Add methods for Fahrenheit/Kelvin conversion
3. **Weather Alerts**: Implement temperature threshold notifications
4. **Batch Operations**: Support for bulk temperature data operations
5. **Data Validation**: Add temperature range validation before saving