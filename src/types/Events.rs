use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

// <T: Debug + Clone + Serialize + Send + Sync>
//#[derive(Clone)]
pub trait EventData: Clone + Debug + Send + Sync {
    fn get_type() -> Events
    where
        Self: Sized;
}

impl<T: EventData> EventData for Vec<T> {
    fn get_type() -> Events
    where
        Self: Sized,
    {
        T::get_type() // This assumes that all elements in the Vec have the same type
    }
}
