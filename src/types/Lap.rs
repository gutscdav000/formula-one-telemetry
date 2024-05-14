/*
 {
    "date_start": "2023-09-16T13:59:07.606000+00:00",
    "driver_number": 63,
    "duration_sector_1": 26.966,
    "duration_sector_2": 38.657,
    "duration_sector_3": 26.12,
    "i1_speed": 307,
    "i2_speed": 277,
    "is_pit_out_lap": false,
    "lap_duration": 91.743,
    "lap_number": 8,
    "meeting_key": 1219,
    "segments_sector_1": [
      2049,
      2049,
      2049,
      2051,
      2049,
      2051,
      2049,
      2049
    ],
    "segments_sector_2": [
      2049,
      2049,
      2049,
      2049,
      2049,
      2049,
      2049,
      2049
    ],
    "segments_sector_3": [
      2048,
      2048,
      2048,
      2048,
      2048,
      2064,
      2064,
      2064
    ],
    "session_key": 9161,
    "st_speed": 298
  }
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Lap {
    pub date_start: String, //TODO: convert to timestamp
    pub driver_number: u32,
    pub duration_sector_1: f32,
    pub duration_sector_2: f32,
    pub duration_sector_3: f32,
    pub i1_speed: u32,
    pub i2_speed: u32,
    pub is_pit_out_lap: bool,
    pub lap_duration: f32,
    pub lap_number: u32,
    pub meeting_key: u32,
    // Dropping these because they'll throw in a null in the array sometimes
    // also this data isn't availabe during the race and IDC rn.
    //    pub segments_sector_1: Vec<u32>,
    //    pub segments_sector_2: Vec<u32>,
    //    pub segments_sector_3: Vec<u32>,
    pub session_key: u32,
    pub st_speed: u32,
}
