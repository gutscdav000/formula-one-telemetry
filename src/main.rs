pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::types::driver::*;
use crate::types::session::Session;
use crate::types::car_data::CarData;

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
}