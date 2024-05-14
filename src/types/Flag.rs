use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum RaceControlError {
    InvalidFlag(String),
}

impl fmt::Display for RaceControlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RaceControlError::InvalidFlag(ref msg) => write!(f, "Invalid flag: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    BlackAndWhite,
    Blue,
    Chequered,
    Clear,
    Green,
    Red,
    Yellow,
    DoubleYellow,
}

impl FromStr for Flag {
    type Err = RaceControlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BLACK AND WHITE" => Ok::<Flag, Self::Err>(Flag::BlackAndWhite),
            "BLUE" => Ok::<Flag, Self::Err>(Flag::Blue),
            "CHEQUERED" => Ok::<Flag, Self::Err>(Flag::Chequered),
            "CLEAR" => Ok::<Flag, Self::Err>(Flag::Clear),
            "GREEN" => Ok::<Flag, Self::Err>(Flag::Green),
            "RED" => Ok::<Flag, Self::Err>(Flag::Red),
            "YELLOW" => Ok::<Flag, Self::Err>(Flag::Yellow),
            "DOUBLE YELLOW" => Ok::<Flag, Self::Err>(Flag::DoubleYellow),
            _ => Err(RaceControlError::InvalidFlag(s.to_owned())),
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_string = match *self {
            Flag::BlackAndWhite => "BLACK AND WHITE",
            Flag::Blue => "BLUE",
            Flag::Chequered => "CHEQUERED",
            Flag::Clear => "CLEAR",
            Flag::Green => "GREEN",
            Flag::Red => "RED",
            Flag::Yellow => "YELLOW",
            Flag::DoubleYellow => "DOUBLE YELLOW",
        };
        write!(f, "{}", formatted_string) // Corrected to output plain string
    }
}

impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Flag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Flag::from_str(&s).map_err(de::Error::custom)
    }
}
