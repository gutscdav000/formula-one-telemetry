/*
{
  "date": "2023-09-15T09:40:43.005000",
  "driver_number": 11,
  "meeting_key": 1219,
  "recording_url": "https://livetiming.formula1.com/static/2023/2023-09-17_Singapore_Grand_Prix/2023-09-15_Practice_1/TeamRadio/SERPER01_11_20230915_104008.mp3",
  "session_key": 9158
}
 */
use crate::types::to_json::ToJson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct TeamRadio {
    pub date: String, //TODO: replace with timestamp
    pub driver_number: u32,
    pub meeting_key: u32,
    pub recording_url: String,
    pub session_key: u32,
}

impl ToJson for Vec<TeamRadio> {
    fn to_json(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }
}
