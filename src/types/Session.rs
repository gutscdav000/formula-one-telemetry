use fred::prelude::*;
/**
[
  {
    "circuit_key": 7,
    "circuit_short_name": "Spa-Francorchamps",
    "country_code": "BEL",
    "country_key": 16,
    "country_name": "Belgium",
    "date_end": "2023-07-29T15:35:00+00:00",
    "date_start": "2023-07-29T15:05:00+00:00",
    "gmt_offset": "02:00:00",
    "location": "Spa-Francorchamps",
    "meeting_key": 1216,
    "session_key": 9140,
    "session_name": "Sprint",
    "session_type": "Race",
    "year": 2023
  }
]
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Session {
    pub circuit_key: u32,
    pub circuit_short_name: String,
    pub country_code: String,
    pub country_key: u32,
    pub country_name: String,
    pub date_end: String, //TODO: replace with date type
    pub date_start: String,
    pub gmt_offset: String, //TODO: replace with time type
    pub location: String,
    pub meeting_key: u32,
    pub session_key: u32,
    pub session_name: String,
    pub session_type: String,
    pub year: u32,
}

impl FromRedis for Session {
    fn from_value(value: RedisValue) -> Result<Self, fred::error::RedisError> {
        // Convert the RedisValue to a JSON string
        println!("Value: {:?}", value);
        let json_str = match value {
            RedisValue::String(s) => s,
            e => {
                return Err(fred::error::RedisError::new(
                    fred::error::RedisErrorKind::InvalidArgument,
                    format!("Expected a string, Received: {:?}", e),
                ))
            }
        };

        // Deserialize the JSON string into a Session struct
        serde_json::from_str(&json_str).map_err(|e| {
            fred::error::RedisError::new(fred::error::RedisErrorKind::Parse, e.to_string())
        })
    }
}
