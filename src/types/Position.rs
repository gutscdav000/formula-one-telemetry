/*
  {
    "date": "2023-08-26T09:30:47.199000+00:00",
    "driver_number": 40,
    "meeting_key": 1217,
    "position": 2,
    "session_key": 9144
  },
*/
use crate::types::events::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Position {
    pub date: String, //TODO: convert to timestamp
    pub driver_number: u32,
    pub meeting_key: u32,
    pub position: u32,
    pub session_key: u32,
}

impl EventData for Position {
    fn get_type() -> Events {
        Events::Position
    }
}
