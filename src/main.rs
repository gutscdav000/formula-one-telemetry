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
    // let _ = event_sync
    //     .position_sync(session.meeting_key, &driver_number, None)
    //     .await;
    //let _ = event_sync.stints_sync(session.session_key, None).await;
    let _ = event_sync
        .run_sync(
            session.session_key,
            session.meeting_key,
            None,
            None,
            driver_number,
            60,
            None,
            None,
            None,
        )
        .await;

    Ok(())
}
