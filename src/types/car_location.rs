/*
  {
    "date": "2023-09-16T13:03:35.292000+00:00",
    "driver_number": 81,
    "meeting_key": 1219,
    "session_key": 9161,
    "x": 567,
    "y": 3195,
    "z": 187
  },
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CarLocation {
    pub date: String, //TODO: convert to timestamp
    pub driver_number: u32,
    pub meeting_key: u32,
    pub session_key: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
