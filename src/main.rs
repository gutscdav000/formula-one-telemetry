pub mod algebras;
pub mod types;
use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::event_sync::EventSync;
use crate::algebras::event_sync::EventSyncImpl;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use crate::types::event_sync::EventSyncConfig;
use crate::types::events::Events;
use crate::types::session::Session;
use log::{error, warn};
use std::error::Error;

//WS
use axum::routing::get;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
// Channel
use std::any::Any;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    //let uri: &'static str = "https://api.openf1.org";
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
    let (channel_tx, mut channel_rx) = broadcast::channel::<Events>(100);

    let event_sync_delay_config: &'static EventSyncConfig = &EventSyncConfig {
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
        tx: channel_tx.clone(),
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
    thread::sleep(Duration::from_millis(5000));
    error!("AFTER EVENT SYNC");

    // WEB SOCKETS
    // TODO: is this for logging or tracing too?
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    //Channel
    let (socket_tx, mut socket_rx) = broadcast::channel::<String>(100);

    // Clone the sender for the interval task
    let channel_tx_clone = channel_tx.clone();

    // Spawn a task to send messages every 2 seconds
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            if let Err(e) = channel_tx_clone.send(Events::Session) {
                eprintln!("Error sending message: {}", e);
            }
        }
    });

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", move |socket: SocketRef| {
        let socket_tx_clone = socket_tx.clone();
        tokio_scoped::scope(|scope| {
            scope.spawn(async move {
                on_connect(
                    &redis_client.clone(),
                    socket,
                    //socket_tx_clone,
                    channel_tx,
                )
                .await;
            });
        });
    });
    let app = axum::Router::new()
        .route("/", get(|| async { "pong" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );
    info!("starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn on_connect(
    redis_client: &RedisImpl,
    socket: SocketRef,
    //socket_tx: broadcast::Sender<T>,
    channel_tx: broadcast::Sender<Events>,
) {
    info!("socket connected: {}", socket.id);

    let mut rx = channel_tx.subscribe();
    let result_json = redis_client
        .get_json::<Vec<CarData>, String>("car_data:4".to_string())
        .await;
    error!("RESULT: {:?}", result_json);
    let json = result_json.ok().flatten().unwrap();

    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            info!("Emitting message: {:?}", message);
            let _ = socket.emit(format!("{:?}", message), &json);
            //let _ = socket.emit("message", "Hello world".to_string());
        }
    });
}
