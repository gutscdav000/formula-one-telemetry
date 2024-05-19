use crate::types::redis::RedisClientError;
use async_trait::async_trait;
use core::fmt::Display;
use fred::prelude::*;
use fred::types::RedisKey;
use log::error;
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
}

pub struct RedisImpl {
    pub client: RedisClient,
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
}
