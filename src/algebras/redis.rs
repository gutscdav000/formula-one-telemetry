use crate::types::redis::RedisClientError;
use async_trait::async_trait;
use core::fmt::Display;
use fred::prelude::*;
use fred::types::RedisKey;
use log::{error, info, warn};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::marker::Send;

#[async_trait]
pub trait Redis {
    async fn set_json<V: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: V,
    ) -> Result<(), RedisClientError>;
    async fn get_json<V: DeserializeOwned, K: Into<RedisKey> + Send + Display + Clone>(
        &self,
        key: K,
    ) -> Result<Option<V>, RedisClientError>;
    async fn redis_fire_and_forget<V: Serialize + Send + Sync>(
        &self,
        maybe_value: Option<Vec<V>>,
        redis_key: String,
    );
}

#[derive(Clone)]
pub struct RedisImpl {
    pub client: RedisClient,
}
impl RedisImpl {
    pub fn default() -> Result<RedisImpl, RedisError> {
        info!("Connecting to redis");
        let config: RedisConfig = RedisConfig::default();
        let reconnect_policy: ReconnectPolicy = ReconnectPolicy::new_exponential(5, 1, 10, 5);
        let client = RedisClient::new(config);
        let _ = client.connect(Some(reconnect_policy));
        let _ = client.wait_for_connect();
        let redis_algebra: RedisImpl = RedisImpl { client: client };
        info!("Connected to Redis");
        Ok(redis_algebra)
    }
}

#[async_trait]
impl Redis for RedisImpl {
    async fn set_json<V: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: V,
    ) -> Result<(), RedisClientError> {
        let json = serde_json::to_string(&value)?;
        self.client.set(key, json, None, None, false).await?;
        Ok(())
    }

    async fn get_json<V, K>(&self, key: K) -> Result<Option<V>, RedisClientError>
    where
        V: DeserializeOwned,
        K: Into<RedisKey> + Send + Display + Clone,
    {
        let key_log = key.clone();
        let json: Option<String> = self.client.get(key).await?;
        if let Some(json_str) = json {
            let value: V = serde_json::from_str(&json_str)?;
            Ok(Some(value))
        } else {
            error!("value not found for key: {}", key_log);
            Ok(None)
        }
    }

    async fn redis_fire_and_forget<V: Serialize + Send + Sync>(
        &self,
        maybe_value: Option<Vec<V>>,
        redis_key: String,
    ) {
        match maybe_value {
            Some(value) => match self.set_json::<Vec<V>>(&redis_key, value).await {
                Ok(_) => info!("{redis_key} synced"),
                Err(e) => error!("could not Redis write {redis_key}, err: {e}"),
            },
            None => warn!("no {redis_key} returned from client"),
        }
    }
}
