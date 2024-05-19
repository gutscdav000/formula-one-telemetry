/*
  {
    "date": "2023-09-15T09:38:23.038000+00:00",
    "driver_number": 63,
    "lap_number": 5,
    "meeting_key": 1219,
    "pit_duration": 24.5,
    "session_key": 9158
  },
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Pit {
    pub date: String, //TODO: convert to timestamp
    pub driver_number: u32,
    pub lap_number: u32,
    pub meeting_key: u32,
    pub pit_duration: Option<f32>,
    pub session_key: u32,
}
