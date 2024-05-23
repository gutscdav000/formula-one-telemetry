use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::events::Events;
use async_trait::async_trait;
use axum::routing::get;
use log::{error, info};
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[async_trait]
pub trait Websocket {
    async fn run(
        self: Arc<Self>,
        channel_tx: broadcast::Sender<Events>,
    ) -> Result<(), Box<dyn Error>>;
    async fn on_connect(&self, socket: SocketRef, channel_tx: broadcast::Sender<Events>);
}
pub struct WebsocketImpl {
    pub redis_client: Arc<RedisImpl>,
}

#[async_trait]
impl Websocket for WebsocketImpl {
    async fn run(
        self: Arc<Self>,
        channel_tx: broadcast::Sender<Events>,
    ) -> Result<(), Box<dyn Error>> {
        let (layer, io) = SocketIo::new_layer();
        io.ns("/", move |socket: SocketRef| {
            tokio_scoped::scope(|scope| {
                scope.spawn(async move {
                    self.on_connect(socket, channel_tx).await;
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

    async fn on_connect(&self, socket: SocketRef, channel_tx: broadcast::Sender<Events>) {
        info!("socket connected: {}", socket.id);

        let mut rx = channel_tx.subscribe();
        let result_json = self
            .redis_client
            .get_json::<Vec<CarData>, String>("car_data:4".to_string())
            .await;
        error!("RESULT: {:?}", result_json);
        let json = result_json.ok().flatten().unwrap();

        tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                info!("Emitting message: {:?}", message);
                let _ = socket.emit(format!("{:?}", message), &json);
            }
        });
    }
}
