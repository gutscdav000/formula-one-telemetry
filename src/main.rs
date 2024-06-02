pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::channel_queue::*;
use crate::algebras::event_sync::EventSync;
use crate::algebras::event_sync::EventSyncImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::redis::RedisImpl;
use crate::algebras::websocket::Websocket;
use crate::algebras::websocket::WebsocketImpl;
use crate::types::driver::*;
use crate::types::event::Event;
use crate::types::event_sync::EventSyncConfig;
use crate::types::session::Session;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let uri: &'static str = Box::leak(Box::new(String::from("https://api.openf1.org"))).as_str();
    let http_requester: &'static TelemetryHttpRequester = &TelemetryHttpRequester;
    let api: &'static CarDataApiImpl = Box::leak(Box::new(CarDataApiImpl {
        http_requester: &http_requester,
        uri: &uri,
    }));

    let sessions: Option<Vec<Session>> =
        api.get_session(&"Italy".to_string(), &"Qualifying".to_string(), 2024);
    println!("Sessions: {:?}", sessions);
    let session: Session = sessions
        .and_then(|vec| vec.clone().pop())
        .expect("Session not found, or request timed out");

    let driver_number = get_driver_number(&DriverName::LandoNorris);
    let redis_client: &'static RedisImpl = Box::leak(Box::new(
        RedisImpl::default().expect("unable to connect to redis"),
    ));
    let (channel_tx, _) = broadcast::channel::<Message>(100);
    let channel_queue = Arc::new(ChannelQueueImpl { tx: channel_tx });

    let event_sync_delay_config: &'static EventSyncConfig = &EventSyncConfig {
        car_data_duration_secs: 2,
        interval_duration_secs: 5,
        team_radio_duration_secs: 30,
        laps_duration_secs: 120,
        pit_duration_secs: 120,
        position_duration_secs: 120,
        stints_duration_secs: 120,
    };
    let event_sync = EventSyncImpl {
        api: &api,
        redis: &redis_client,
        delay_config: &event_sync_delay_config,
        tx: channel_queue.clone(),
    };

    tokio::spawn(async move {
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
    });
    // this is here to give the event_sync a moment to populate the cache
    thread::sleep(Duration::from_millis(5000));

    info!("Begin Websocket streaming");
    // TODO: is this for logging or tracing too?
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let _ = Arc::new(WebsocketImpl {
        redis_client: Arc::new(redis_client.clone()),
        channel_tx: channel_queue,
    })
    .run()
    .await;

    Ok(())
}
