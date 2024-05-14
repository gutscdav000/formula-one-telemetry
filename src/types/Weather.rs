/*
 {
   "air_temperature": 27.8,
   "date": "2023-05-07T18:42:25.233000+00:00",
   "humidity": 58,
   "meeting_key": 1208,
   "pressure": 1018.7,
   "rainfall": 0,
   "session_key": 9078,
   "track_temperature": 52.5,
   "wind_direction": 136,
   "wind_speed": 2.4
 }
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Weather {
    pub air_temperature: f32,
    pub date: String, //TODO: make a timestamp
    pub humidity: f32,
    pub meeting_key: u32,
    pub pressure: f32,
    pub rainfall: u32,
    pub session_key: u32,
    pub track_temperature: f32,
    pub wind_direction: u32,
    pub wind_speed: f32,
}
