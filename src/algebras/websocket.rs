use crate::algebras::channel_queue::*;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::event::Event;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::pit::Pit;
use crate::types::position::Position;
use crate::types::redis::RedisClientError;
use crate::types::stint::Stint;
use crate::types::team_radio::TeamRadio;
use crate::types::to_json::ToJson;
use async_trait::async_trait;
use axum::routing::get;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use log::{error, info};
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[async_trait]
pub trait Websocket {
    async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>>;
    async fn route_event(self: Arc<Self>, msg: &Message) -> Option<String>;
    async fn on_connect(self: Arc<Self>, socket: SocketRef);
    async fn cache_prefetch(self: Arc<Self>, socket: Arc<TokioMutex<SocketRef>>);
}
pub struct WebsocketImpl {
    pub redis_client: Arc<RedisImpl>,
    pub channel_tx: Arc<dyn ChannelQueue + Send + Sync>,
}

#[async_trait]
impl Websocket for WebsocketImpl {
    async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        let (layer, io) = SocketIo::new_layer();
        io.ns("/", move |socket: SocketRef| {
            tokio_scoped::scope(|scope| {
                scope.spawn(async move {
                    self.on_connect(socket /*, channel_tx*/).await;
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
            Event::Lap => parse_redis_result::<Vec<Lap>>(
                self.redis_client
                    .get_json::<Vec<Lap>, String>("laps".to_string())
                    .await,
            ),
            Event::Pit => parse_redis_result::<Vec<Pit>>(
                self.redis_client
                    .get_json::<Vec<Pit>, String>("pits".to_string())
                    .await,
            ),
            Event::Position => parse_redis_result::<Vec<Position>>(
                self.redis_client
                    .get_json::<Vec<Position>, String>("position:4".to_string())
                    .await,
            ),
            Event::Stint => parse_redis_result::<Vec<Stint>>(
                self.redis_client
                    .get_json::<Vec<Stint>, String>("stints".to_string())
                    .await,
            ),
            Event::TeamRadio => parse_redis_result::<Vec<TeamRadio>>(
                self.redis_client
                    .get_json::<Vec<TeamRadio>, String>("team_radio".to_string())
                    .await,
            ),
            //TODO: Not yet implemented
            Event::Session => None,
        }
    }

    async fn on_connect(self: Arc<Self>, socket: SocketRef) {
        info!("socket connected: {}", socket.id);

        //TODO: prefetch here
        // error!("CACHE PREFETCH");
        // let socket_mutex = Arc::new(TokioMutex::new(socket));
        // let _ = self.clone().cache_prefetch(Arc::clone(&socket_mutex)).await;
        error!("CACHE PREFETCH");
        let socket_mutex = Arc::new(TokioMutex::new(socket));
        let _ = self.clone().cache_prefetch(Arc::clone(&socket_mutex)).await;

        let mut rx = self.channel_tx.subscribe();
        let socket_mutex_clone = Arc::clone(&socket_mutex);

        let mut rx = self.channel_tx.subscribe();
        tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                let s = self.clone().route_event(&message).await;
                let sock = socket_mutex_clone.lock().await;
                info!("Emitting message: {:?}", message);
                s.clone()
                    .and_then(|s| Some(sock.emit(format!("{}", message), &s)));
            }
        });
    }

    async fn cache_prefetch(self: Arc<Self>, socket: Arc<TokioMutex<SocketRef>>) {
        let futures: FuturesUnordered<_> = Event::all_variants()
            .into_iter()
            .map(|event| {
                let self_clone = Arc::clone(&self);
                let socket_clone = Arc::clone(&socket);
                async move {
                    let message = Message { msg: event };
                    let maybe_json = self_clone.route_event(&message).await;
                    let socket_lock = socket_clone.lock().await;
                    let _ = maybe_json.map(|json| socket_lock.emit(format!("{}", message), &json));
                }
            })
            .collect();

        futures.for_each(|_| async {}).await;
    }
}

fn parse_redis_result<T: ToJson>(result: Result<Option<T>, RedisClientError>) -> Option<String> {
    result.ok().flatten().and_then(|t| t.to_json())
}
