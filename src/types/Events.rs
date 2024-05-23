use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Events {
    CarData,
    Interval,
    Lap,
    Pit,
    Position,
    Session,
    Stint,
    TeamRadio,
}
