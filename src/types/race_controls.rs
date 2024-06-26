/*
 {
   "category": "Flag",
   "date": "2023-06-04T14:21:01+00:00",
   "driver_number": 1,
   "flag": "BLACK AND WHITE",
   "lap_number": 59,
   "meeting_key": 1211,
   "message": "BLACK AND WHITE FLAG FOR CAR 1 (VER) - TRACK LIMITS",
   "scope": "Driver",
   "sector": null,
   "session_key": 9102
 }
*/
use crate::types::driver::*;
use crate::types::flag::Flag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RaceControl {
    pub category: Category,
    pub date: String, //TODO: make timestamps
    pub driver_number: DriverNumber,
    pub flag: Flag,
    pub lap_number: u32,
    pub meeting_key: u32,
    pub message: String,
    pub scope: String, // should be enum? not sure
    pub sector: Option<u32>,
    pub session_key: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Category {
    Flag,
    Other,
    Drs,
    SafetyCar,
    CarEvent,
}
