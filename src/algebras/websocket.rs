use crate::algebras::channel_queue::*;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::events::*;
use async_trait::async_trait;
use axum::routing::get;
use log::{error, info};
use serde::Serialize;
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[async_trait]
pub trait Websocket<T: Debug + Clone + EventData + Serialize + Send + Sync + 'static> {
    async fn run(
        self: Arc<Self>,
        channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
    ) -> Result<(), Box<dyn Error>>;
    //    async fn route_event(self: Arc<Self>, event: &T) -> String;
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
impl<T: Debug + Clone + EventData + Send + Serialize + Sync + 'static> Websocket<T>
    for WebsocketImpl
{
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

    //TODO: Is there a better way to separate concerns than including this in the algebra?
    //      One option is to send json through the channel from event_sync... probably has a performance overhead
    //    could we have 2 implementations? 1) redis, 2) send json by channel?
    //    define common methods in the trait, and 2 separate impls for methods that diverge
    // async fn route_event(self: Arc<Self>, event: &T) -> String {
    //     match event {
    //         Events::CarData => {
    //             error!("ROUTING EVENT");

    //             //TODO: this is hot garbage fix it...
    //             let cd = self
    //                 .redis_client
    //                 .get_json::<Vec<CarData>, String>("car_data:4".to_string())
    //                 .await
    //                 .ok()
    //                 .flatten()
    //                 .unwrap();

    //             //TODO: separation of concerns, could a trait be made for CarData and associated types
    //             serde_json::to_string(&cd).unwrap()
    //         }
    //         _ => "".to_string(),
    //     }
    // }

    async fn on_connect(
        self: Arc<Self>,
        socket: SocketRef,
        channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
    ) {
        info!("socket connected: {}", socket.id);

        //let mut rx = channel_tx.subscribe();
        let mut rx = Arc::as_ref(&channel_tx).subscribe();
        let self_clone = Arc::clone(&self);

        tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                //let s = self.clone().route_event(&message).await;
                //let s = self_clone.route_event(&Events::CarData).await;
                info!("Emitting message: {:?}", message);
                //let _ = socket.emit(format!("{:?}", message), &s);
                let _ = socket.emit("message", message.value);
            }
        });
    }
}
