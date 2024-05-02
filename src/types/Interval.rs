/*
[
  {
    "date": "2023-09-17T13:31:02.395000+00:00",
    "driver_number": 1,
    "gap_to_leader": 41.019,
    "interval": 0.003,
    "meeting_key": 1219,
    "session_key": 9165
  }
]
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Interval {
    pub gap_to_leader: Option<f32>,
    pub interval: Option<f32>,
    pub driver_number: u32,
    pub date: String, //TODO: make this a timestamp
    pub session_key: u32,
    pub meeting_key: u32,
}

    
