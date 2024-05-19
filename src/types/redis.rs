use fred::prelude::RedisError;
use serde_json;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RedisClientError {
    JsonDecodingFailure(serde_json::Error),
    RedisError(RedisError),
}

impl fmt::Display for RedisClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RedisClientError::JsonDecodingFailure(ref e) => write!(f, "JsonDecodingFailure: {}", e),
            RedisClientError::RedisError(ref e) => write!(f, "RedisError: {}", e),
        }
    }
}

impl From<RedisError> for RedisClientError {
    fn from(error: RedisError) -> Self {
        RedisClientError::RedisError(error)
    }
}

impl From<serde_json::Error> for RedisClientError {
    fn from(error: serde_json::Error) -> Self {
        RedisClientError::JsonDecodingFailure(error)
    }
}

impl Error for RedisClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
