/*
{
  "compound": "SOFT",
  "driver_number": 16,
  "lap_end": 20,
  "lap_start": 1,
  "meeting_key": 1219,
  "session_key": 9165,
  "stint_number": 1,
  "tyre_age_at_start": 3
}
*/
use crate::types::to_json::ToJson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Stint {
    pub compound: String,
    pub driver_number: u32,
    pub lap_end: u32,
    pub lap_start: u32,
    pub meeting_key: u32,
    pub session_key: u32,
    pub stint_number: u32,
    pub tyre_age_at_start: u32,
}

impl ToJson for Vec<Stint> {
    fn to_json(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }
}
