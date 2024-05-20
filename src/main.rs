pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::event_sync::EventSync;
use crate::algebras::event_sync::EventSyncImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::redis::RedisImpl;
use crate::types::driver::*;
use crate::types::event_sync::EventSyncConfig;
use crate::types::session::Session;
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

    let event_sync_delay_config = EventSyncConfig {
        car_data_duration_secs: 2,
        interval_duration_secs: 5,
        team_radio_duration_secs: 30,
        laps_duration_secs: 120,
        pit_duration_secs: 120,
        position_duration_secs: 5, //TODO: probably higher if we're not using this...
        stints_duration_secs: 120,
    };
    let event_sync = EventSyncImpl {
        api: &api,
        redis: &redis_client,
        delay_config: &event_sync_delay_config,
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
