pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::types::car_data::CarData;
use crate::types::car_location::CarLocation;
use crate::types::driver::*;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::meeting::Meeting;
use crate::types::session::Session;

fn main() {
    println!("Hello, world!");
    let http_requester = TelemetryHttpRequester;
    let api = CarDataApiImpl{
	http_requester: &http_requester,
	uri: "https://api.openf1.org",
    };
    
    let sessions: Option<Vec<Session>> = api.get_session(&"Belgium".to_string(), &"Sprint".to_string(), 2023);
    println!("Sessions: {:?}", sessions);

    let driver_number = get_driver_number(&DriverName::MaxVerstappen);
    let session: Session = sessions.unwrap().pop().unwrap();
    let drivers: Option<Vec<Driver>> = api.get_drivers(session.session_key, &driver_number);
    println!("Drivers: {:?}", drivers);

    let car_data: Option<Vec<CarData>> = api.get_car_data(session.session_key, &driver_number, Some(315));
    println!("CarData: {:?}", car_data);

    let interv: Option<f32> = Some(0.01f32);
    let interval: Option<Vec<Interval>> = api.get_intervals(session.session_key, interv);
    println!("Interval: {:?}", interval);

    let laps: Option<Vec<Lap>> = api.get_lap(session.session_key, &driver_number, 8);
    println!("Laps: {:?}", laps);

    let car_loc_driver = get_driver_number(&DriverName::OscarPiastri);
    let car_locations: Option<Vec<CarLocation>> = api.get_car_location(9161, &car_loc_driver, &"2023-09-16T13:03:35.200", &"2023-09-16T13:03:35.800");
    println!("CarLocations: {:?}", car_locations);

    let meeting: Option<Vec<Meeting>> = api.get_meeting(2023, &"Singapore");
    println!("Meeting: {:?}", meeting);
    
}
