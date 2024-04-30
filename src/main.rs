pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
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
}
