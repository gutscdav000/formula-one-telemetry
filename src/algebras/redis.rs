use crate::types::redis::*;
use async_trait::async_trait;
use core::fmt::Display;
use fred::prelude::*;
use fred::types::RedisKey;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::marker::Send;
use std::marker::Unpin;

#[async_trait]
pub trait Redis {
    async fn set_json<V: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &V,
    ) -> Result<(), RedisClientError>;
    async fn get_json<
        V: DeserializeOwned + FromRedis + Unpin + Send + Sync + 'static,
        K: Into<RedisKey> + Send + Display,
    >(
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
        value: &V,
    ) -> Result<(), RedisClientError> {
        let json = serde_json::to_string(value)?;
        self.client.set(key, json, None, None, false).await?;
        Ok(())
    }

    //TODO: change this implementation, it licks butthole
    async fn get_json<V, K>(&self, key: K) -> Result<Option<V>, RedisClientError>
    where
        V: DeserializeOwned + FromRedis + Unpin + Send + Sync + 'static,
        K: Into<RedisKey> + Send,
    {
        let json: V = self.client.get::<V, K>(key).await?;
        Ok(Some(json))
        // if let Some(json_str) = json {
        //     let value = serde_json::from_str(&json_str)?;
        //     Ok(Some(value))
        // } else {
        //     Ok(None)
        // }
    }
}
