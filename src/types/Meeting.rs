/*
  {
    "circuit_key": 61,
    "circuit_short_name": "Singapore",
    "country_code": "SGP",
    "country_key": 157,
    "country_name": "Singapore",
    "date_start": "2023-09-15T09:30:00+00:00",
    "gmt_offset": "08:00:00",
    "location": "Marina Bay",
    "meeting_key": 1219,
    "meeting_name": "Singapore Grand Prix",
    "meeting_official_name": "FORMULA 1 SINGAPORE AIRLINES SINGAPORE GRAND PRIX 2023",
    "year": 2023
  }
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Meeting {
    pub circuit_key: u32,
    pub circuit_short_name: String,
    pub country_code: String,
    pub country_key: u32,
    pub country_name: String,
    pub date_start: String, //TODO: make timestamp
    pub gmt_offset: String, //TODO: make time
    pub location: String,
    pub meeting_key: u32,
    pub meeting_name: String,
    pub meeting_official_name: String,
    pub year: u32
  }
