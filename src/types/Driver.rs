/**
  {
    "broadcast_name": "M VERSTAPPEN",
    "country_code": "NED",
    "driver_number": 1,
    "first_name": "Max",
    "full_name": "Max VERSTAPPEN",
    "headshot_url": "https://www.formula1.com/content/dam/fom-website/drivers/M/MAXVER01_Max_Verstappen/maxver01.png.transform/1col/image.png",
    "last_name": "Verstappen",
    "meeting_key": 1219,
    "name_acronym": "VER",
    "session_key": 9158,
    "team_colour": "3671C6",
    "team_name": "Red Bull Racing"
  }
 */
use std::fmt;
use std::collections::HashMap;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Driver {
    pub driver_number: u32,
    pub broadcast_name: String,
    pub full_name: String,
    pub name_acronym: String,
    pub session_key: u32,
    pub meeting_key: Option<u32>,
    pub team_name: Option<String>,
    pub first_name: Option<String>,
    pub team_colour: Option<String>,
    pub country_code: Option<String>,
    pub last_name: Option<String>,
    pub headshot_url: Option<String>,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum DriverName {
    MaxVerstappen,
    LoganSargeant,
    DanielRicciardo,
    LandoNorris,
    PierreGasly,
    SergioPerez,
    FernandoAlonso,
    CharlesLeclerc,
    LanceStroll,
    KevinMagnussen,
    YukiTsunoda,
    AlexAlbon,
    ZhouGuanyu,
    NicoHulkenberg,
    EstebanOcon,
    LewisHamilton,
    CarlosSainz,
    GeorgeRussell,
    ValtteriBottas,
    OscarPiastri,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub struct DriverNumber(#[serde(rename = "driver_number")] u32);
impl fmt::Display for DriverNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl DriverNumber {
    pub fn new(num: u32) -> Self {
	DriverNumber(num)
    }
}

pub fn get_driver_number(driver_name: &DriverName) -> DriverNumber {
    let driver_map: HashMap<DriverName, DriverNumber> = HashMap::from([
	(DriverName::MaxVerstappen, DriverNumber(1)),
	(DriverName::LoganSargeant, DriverNumber(2)),
	(DriverName::DanielRicciardo, DriverNumber(3)),
	(DriverName::LandoNorris, DriverNumber(4)),
	(DriverName::PierreGasly, DriverNumber(10)),
	(DriverName::SergioPerez, DriverNumber(11)),
	(DriverName::FernandoAlonso, DriverNumber(14)),
	(DriverName::CharlesLeclerc, DriverNumber(16)),
	(DriverName::LanceStroll, DriverNumber(18)),
	(DriverName::KevinMagnussen, DriverNumber(20)),
	(DriverName::YukiTsunoda, DriverNumber(22)),
	(DriverName::AlexAlbon, DriverNumber(23)),
	(DriverName::ZhouGuanyu, DriverNumber(24)),
	(DriverName::NicoHulkenberg, DriverNumber(27)),
	(DriverName::EstebanOcon, DriverNumber(31)),
	(DriverName::LewisHamilton, DriverNumber(44)),
	(DriverName::CarlosSainz, DriverNumber(55)),
	(DriverName::GeorgeRussell, DriverNumber(63)),
	(DriverName::ValtteriBottas, DriverNumber(77)),
	(DriverName::OscarPiastri, DriverNumber(81)),
    ]);
    
    *driver_map.get(driver_name).unwrap()
}


