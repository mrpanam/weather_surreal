# Sunset Table Implementation

This document explains the sunset table functionality that saves partial weather data from the OpenWeatherMap API to SurrealDB.

## Overview

The sunset table stores sunrise and sunset times for cities, extracted from the full weather response. This demonstrates how to save only specific fields from a larger API response to a dedicated table.

## Data Structure

The `Sun` struct represents the data stored in the `sunset` table:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Sun {
    pub city_id: String,    // Simple string identifier for the city
    pub sunrise: u64,       // Unix timestamp for sunrise
    pub sunset: u64,        // Unix timestamp for sunset
    pub date: u64,          // Unix timestamp for the date
}
```

## Key Features

### 1. Partial Data Extraction
The system extracts only sunrise/sunset data from the full weather response:
- `sunrise` and `sunset` from `weather_response.sys`
- `date` from `weather_response.dt`
- `city_id` created as a string: `format!("{}_{}", data.name, data.id)`

### 2. Database Operations
- **Save**: `save_sunset_data(&self, data: &WeatherResponse)`
- **Retrieve All**: `get_all_sunset_data(&self)`
- **Retrieve by City**: `get_sunset_data(&self, city_name: &str)`

### 3. Formatting Utilities
The `Sun` struct includes methods for human-readable formatting:
- `format_sunrise()` - Formatted sunrise time
- `format_sunset()` - Formatted sunset time
- `format_date()` - Formatted date
- `format_summary()` - Complete summary

## Usage

### Running the Application

```bash
# Run the main application
cargo run
```

### Example Output

```
Weather response received for 8 cities
connected to DB
Sunset data saved successfully for London
Sunset data saved successfully for Tokyo
Sunset data saved successfully for Paris
...

=== Retrieving Sunset Data ===
Retrieved 8 sunset records

City: London_2643743
Date: 2025-06-06
Sunrise: 2025-06-06 03:45:44 UTC
Sunset: 2025-06-06 20:12:59 UTC

City: Tokyo_1850144
Date: 2025-06-06
Sunrise: 2025-06-05 19:25:39 UTC
Sunset: 2025-06-06 09:54:22 UTC
...
```

## Database Schema

The `sunset` table in SurrealDB contains records like:

```json
{
    "id": "sunset:⟨record_id⟩",
    "city_id": "London_2643743",
    "sunrise": 1749181544,
    "sunset": 1749240779,
    "date": 1749214213
}
```

## SurrealDB Queries

You can query the sunset table directly using SurrealQL:

```sql
-- Get all sunset records
SELECT * FROM sunset;

-- Get sunset data for a specific city
SELECT * FROM sunset WHERE city_id CONTAINS 'London';

-- Get sunset records for a specific date range
SELECT * FROM sunset WHERE date > 1749214200 AND date < 1749300600;

-- Get cities with sunset after a specific time
SELECT * FROM sunset WHERE sunset > 1749235000;
```

## Implementation Details

### Data Extraction

```rust
pub async fn save_sunset_data(&self, data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
    // Create a city ID string using the city name and ID
    let city_id = format!("{}_{}", data.name, data.id);
    
    // Create Sun struct with partial data from weather response
    let sun_data = Sun {
        city_id,
        sunrise: data.sys.sunrise,
        sunset: data.sys.sunset,
        date: data.dt,
    };

    // Save to database
    DB.create::<Option<Sun>>("sunset").content(sun_data).await
}
```

### Data Retrieval

```rust
// Get all sunset data
let all_sunset_data = db.get_all_sunset_data().await?;

// Get sunset data for a specific city
let city_sunset_data = db.get_sunset_data("London").await?;
```

## Benefits

1. **Simple String IDs**: Easy to work with and query
2. **Selective Data Storage**: Only store the data you need
3. **Optimized Queries**: Faster queries on smaller, focused tables
4. **Data Normalization**: Separate concerns between full weather data and sun times
5. **Flexible Retrieval**: Multiple ways to query the data

## Error Handling

The system includes robust error handling:
- Database connection errors
- Serialization/deserialization errors
- Query execution errors

All database operations return `Result<T, Box<dyn Error>>` for proper error handling.

## Maintenance

If you need to clear the sunset table for any reason:

```rust
// Clear all sunset records (use with caution)
db.clear_sunset_table().await?;
```

This is useful for schema changes or data cleanup operations.