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
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let http_requester = TelemetryHttpRequester;
    let api = CarDataApiImpl {
        http_requester: &http_requester,
        uri: "https://api.openf1.org",
    };

    let sessions: Option<Vec<Session>> =
        api.get_session(&"Italy".to_string(), &"Qualifying".to_string(), 2024);
    println!("Sessions: {:?}", sessions);
    let session: Session = sessions
        .and_then(|vec| vec.clone().pop())
        .expect("Session not found, or request timed out");

    // let driver_number = get_driver_number(&DriverName::LandoNorris);
    //     let redis_client: RedisImpl = RedisImpl::default().expect("unable to connect to redis");

    // let event_sync_delay_config = EventSyncConfig {
    //     car_data_duration_secs: 2,
    //     interval_duration_secs: 5,
    //     team_radio_duration_secs: 30,
    //     laps_duration_secs: 120,
    //     pit_duration_secs: 120,
    //     position_duration_secs: 5, //TODO: probably higher if we're not using this...
    //     stints_duration_secs: 120,
    // };
    // let event_sync = EventSyncImpl {
    //     api: &api,
    //     redis: &redis_client,
    //     delay_config: &event_sync_delay_config,
    // };
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
    // let _ = event_sync
    //     .run_sync(
    //         session.session_key,
    //         session.meeting_key,
    //         None,
    //         None,
    //         driver_number,
    //         60,
    //         None,
    //         None,
    //         None,
    //     )
    //     .await;

    // WEB SOCKETS
    // TODO: is this for logging or tracing too?
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    //Channel
    let (tx, mut rx) = broadcast::channel::<String>(100);
    // Clone the sender for the interval task
    let tx_clone = tx.clone();

    // Spawn a task to send messages every 2 seconds
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            if let Err(e) = tx_clone.send("Hello from server".to_string()) {
                eprintln!("Error sending message: {}", e);
            }
        }
    });

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", move |socket: SocketRef| {
        let tx = tx.clone();
        tokio::spawn(async move {
            on_connect(socket, tx).await;
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

async fn on_connect(socket: SocketRef, tx: broadcast::Sender<String>) {
    info!("socket connected: {}", socket.id);

    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            info!("Emitting message: {}", message);
            let _ = socket.emit("message", &message); //was awaited
        }
    });
}
