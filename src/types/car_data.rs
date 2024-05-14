use serde::{Deserialize, Serialize};
/**
  [{
    "brake": 0,
    "date": "2023-09-15T13:08:19.923000+00:00",
    "driver_number": 55,
    "drs": 12,
    "meeting_key": 1219,
    "n_gear": 8,
    "rpm": 11141,
    "session_key": 9159,
    "speed": 315,
    "throttle": 99
  },]
*/
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CarData {
    pub brake: u32,
    pub date: String, //TODO: make timestamp
    pub driver_number: u32,
    pub drs: u32,
    pub meeting_key: u32,
    pub n_gear: u32,
    pub rpm: u32,
    pub session_key: u32,
    pub speed: u32,
    pub throttle: u32,
}
