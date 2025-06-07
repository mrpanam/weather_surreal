use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct City {
    pub name: String,
    pub lon: f32,
    pub lat: f32,
}

pub fn get_cities() -> Vec<City> {
    vec![
        // Original Cities
        City {
            name: "New York".to_string(),
            lon: -74.0060,
            lat: 40.7128,
        },
        City {
            name: "London".to_string(),
            lon: -0.1278,
            lat: 51.5074,
        },
        City {
            name: "Tokyo".to_string(),
            lon: 139.6917,
            lat: 35.6895,
        },
        City {
            name: "Paris".to_string(),
            lon: 2.3522,
            lat: 48.8566,
        },
        City {
            name: "Sydney".to_string(),
            lon: 151.2093,
            lat: -33.8688,
        },
        City {
            name: "Beijing".to_string(),
            lon: 116.4074,
            lat: 39.9042,
        },
        City {
            name: "Moscow".to_string(),
            lon: 37.6173,
            lat: 55.7558,
        },
        City {
            name: "Rome".to_string(),
            lon: 12.4964,
            lat: 41.9028,
        },
        City {
            name: "Berlin".to_string(),
            lon: 13.4050,
            lat: 52.5200,
        },
        // New: Very Hot
        City {
            name: "Kuwait City".to_string(),
            lon: 47.9774,
            lat: 29.3759,
        },
        City {
            name: "Death Valley".to_string(),
            lon: -116.8656,
            lat: 36.4623,
        },
        City {
            name: "Ahvaz".to_string(),
            lon: 48.6692,
            lat: 31.3203,
        },
        // New: Very Cold
        City {
            name: "Yakutsk".to_string(),
            lon: 129.6755,
            lat: 62.0355,
        },
        City {
            name: "Barrow".to_string(), // Utqiaġvik
            lon: -156.7886,
            lat: 71.2906,
        },
        // New: Wet & Humid
        City {
            name: "Singapore".to_string(),
            lon: 103.8198,
            lat: 1.3521,
        },
        City {
            name: "Cherrapunji".to_string(),
            lon: 91.7308,
            lat: 25.2843,
        },
        // New: Dry
        City {
            name: "Lima".to_string(),
            lon: -77.0428,
            lat: -12.0464,
        },
        City {
            name: "San Pedro de Atacama".to_string(),
            lon: -68.1992,
            lat: -22.9087,
        },
        // New: Variable
        City {
            name: "Reykjavík".to_string(),
            lon: -21.8954,
            lat: 64.1355,
        },
        City {
            name: "Denver".to_string(),
            lon: -104.9903,
            lat: 39.7392,
        },
        // New: Other Interesting
        City {
            name: "Tromsø".to_string(),
            lon: 18.9560,
            lat: 69.6496,
        },
        City {
            name: "Cape Town".to_string(),
            lon: 18.4241,
            lat: -33.9249,
        },
        // Added Cities
        City {
            name: "Mimizan".to_string(),
            lon: -1.2781,
            lat: 44.2017,
        },
        City {
            name: "Biskra".to_string(),
            lon: 5.7281,
            lat: 34.8504,
        },
        City {
            name: "Brest".to_string(),
            lon: -4.4861,
            lat: 48.3904,
        },
        City {
            name: "Marseille".to_string(),
            lon: 5.3698,
            lat: 43.2965,
        },
        City {
            name: "Málaga".to_string(),
            lon: -4.4214,
            lat: 36.7213,
        },
        City {
            name: "Madrid".to_string(),
            lon: -3.7038,
            lat: 40.4168,
        },
        City {
            name: "Barcelona".to_string(),
            lon: 2.1734,
            lat: 41.3851,
        },
        City {
            name: "Marrakesh".to_string(),
            lon: -7.9811,
            lat: 31.6295,
        },
        City {
            name: "New Delhi".to_string(),
            lon: 77.2090,
            lat: 28.6139,
        },
        City {
            name: "Riyadh".to_string(),
            lon: 46.6753,
            lat: 24.7136,
        },
    ]
}
