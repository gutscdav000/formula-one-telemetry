pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::event_sync::EventSync;
use crate::algebras::event_sync::EventSyncImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::car_location::CarLocation;
use crate::types::driver::*;
use crate::types::flag::*;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::meeting::Meeting;
use crate::types::pit::Pit;
use crate::types::position::Position;
use crate::types::race_controls::*;
use crate::types::session::Session;
use crate::types::stint::Stint;
use crate::types::team_radio::TeamRadio;
use crate::types::weather::Weather;
use fred::prelude::*;
use fred::types::RedisConfig;
use fred::types::*;
use log::info;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let http_requester = TelemetryHttpRequester;
    let api = CarDataApiImpl {
        http_requester: &http_requester,
        uri: "https://api.openf1.org",
    };

    let redis_client: RedisImpl = RedisImpl::default().expect("unable to connect to redis");

    let event_sync = EventSyncImpl {
        api: &api,
        redis: &redis_client,
    };

    let sessions: Option<Vec<Session>> =
        api.get_session(&"Italy".to_string(), &"Qualifying".to_string(), 2024);
    println!("Sessions: {:?}", sessions);
    let session: Session = sessions
        .and_then(|vec| vec.clone().pop())
        .expect("Session not found, or request timed out");

    let driver_number = get_driver_number(&DriverName::LandoNorris);
    // let _ = event_sync
    //     .car_data_sync(session.session_key, Some(driver_number), Some(50))
    //     .await;
    //let _ = event_sync.intervals_sync(session.session_key, None).await;
    // let _ = event_sync
    //     .team_radio_sync(session.session_key, Some(driver_number))
    //     .await;
    // let _ = event_sync
    //     .laps_sync(session.session_key, &driver_number, 1)
    //     .await;
    // let _ = event_sync.pit_sync(session.session_key, None).await;
    let _ = event_sync
        .position_sync(session.meeting_key, &driver_number, None)
        .await;
    Ok(())
}

fn test_requests() {
    let http_requester = TelemetryHttpRequester;
    let api = CarDataApiImpl {
        http_requester: &http_requester,
        uri: "https://api.openf1.org",
    };

    let sessions: Option<Vec<Session>> =
        api.get_session(&"Belgium".to_string(), &"Sprint".to_string(), 2023);
    println!("Sessions: {:?}", sessions);

    let driver_number = get_driver_number(&DriverName::MaxVerstappen);
    let session: Session = sessions.unwrap().pop().unwrap();
    let drivers: Option<Vec<Driver>> = api.get_drivers(session.session_key, &driver_number);
    println!("Drivers: {:?}", drivers);

    let car_data: Option<Vec<CarData>> =
        api.get_car_data(session.session_key, Some(driver_number), Some(315));
    //println!("CarData: {:?}", car_data);

    let interv: Option<f32> = Some(0.01f32);
    let interval: Option<Vec<Interval>> = api.get_intervals(session.session_key, interv);
    println!("Interval: {:?}", interval);

    let laps: Option<Vec<Lap>> = api.get_lap(session.session_key, &driver_number, 8);
    println!("Laps: {:?}", laps);

    let car_loc_driver = get_driver_number(&DriverName::OscarPiastri);
    let car_locations: Option<Vec<CarLocation>> = api.get_car_location(
        9161,
        &car_loc_driver,
        &"2023-09-16T13:03:35.200",
        &"2023-09-16T13:03:35.800",
    );
    println!("CarLocations: {:?}", car_locations);

    let meeting: Option<Vec<Meeting>> = api.get_meeting(2023, &"Singapore");
    println!("Meeting: {:?}", meeting);

    let pit: Option<Vec<Pit>> = api.get_pit(9158, Some(25));
    println!("Pit: {:?}", pit);

    let position: Option<Vec<Position>> = api.get_position(1217, &driver_number, Some(1));
    println!("Position: {:?}", position);
    let race_control: Option<Vec<RaceControl>> = api.get_race_control(
        Some(Category::Flag),
        Some(Flag::BlackAndWhite),
        Some(driver_number),
        Some("2023-01-01".to_string()),
        Some("2023-09-01".to_string()),
    );
    println!("RaceControl: {:?}", race_control);

    let stints: Option<Vec<Stint>> = api.get_stints(9165, Some(3));
    println!("Stint: {:?}", stints);

    let team_radio: Option<Vec<TeamRadio>> = api.get_team_radio(9158, Some(driver_number));
    println!("TeamRadio: {:?}", team_radio);

    let weather: Option<Vec<Weather>> = api.get_weather(1208, Some(130), Some(52));
    println!("Weather: {:?}", weather);
}
