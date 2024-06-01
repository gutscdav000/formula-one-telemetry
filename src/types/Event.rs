use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Event {
    CarData,
    Interval,
    Lap,
    Pit,
    Position,
    Session,
    Stint,
    TeamRadio,
}

impl Event {
    pub fn all_variants() -> Vec<Event> {
        vec![
            Event::CarData,
            Event::Interval,
            Event::Lap,
            Event::Pit,
            Event::Position,
            Event::Session,
            Event::Stint,
            Event::TeamRadio,
        ]
    }
}
