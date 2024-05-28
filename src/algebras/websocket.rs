use crate::algebras::channel_queue::*;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::event::Event;
use crate::types::interval::Interval;
use crate::types::redis::RedisClientError;
use crate::types::to_json::ToJson;
use async_trait::async_trait;
use axum::routing::get;
use log::{error, info};
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;
use std::error::Error;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[async_trait]
pub trait Websocket {
    async fn run(
        self: Arc<Self>,
        channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
    ) -> Result<(), Box<dyn Error>>;
    async fn route_event(self: Arc<Self>, msg: &Message) -> Option<String>;
    async fn on_connect(
        self: Arc<Self>,
        socket: SocketRef,
        channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
    );
}
pub struct WebsocketImpl {
    pub redis_client: Arc<RedisImpl>,
}

#[async_trait]
impl Websocket for WebsocketImpl {
    async fn run(
        self: Arc<Self>,
        channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
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

    async fn route_event(self: Arc<Self>, msg: &Message) -> Option<String> {
        let event: &Event = &msg.msg;
        match event {
            Event::CarData => parse_redis_result::<Vec<CarData>>(
                self.redis_client
                    .get_json::<Vec<CarData>, String>("car_data:4".to_string())
                    .await,
            ),
            Event::Interval => parse_redis_result::<Vec<Interval>>(
                self.redis_client
                    .get_json::<Vec<Interval>, String>("intervals".to_string())
                    .await,
            ),
            _ => None,
        }
    }

    async fn on_connect(
        self: Arc<Self>,
        socket: SocketRef,
        channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
    ) {
        info!("socket connected: {}", socket.id);

        let mut rx = channel_tx.subscribe();

        tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                let s = self.clone().route_event(&message).await;
                info!("Emitting message: {:?}", message);
                s.clone()
                    .and_then(|s| Some(socket.emit(format!("{:?}", message), &s)));
            }
        });
    }
}

fn parse_redis_result<T: ToJson>(result: Result<Option<T>, RedisClientError>) -> Option<String> {
    result.ok().flatten().and_then(|t| t.to_json())
}
