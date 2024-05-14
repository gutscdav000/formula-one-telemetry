use anyhow::anyhow;
use attohttpc::{self};
use log::{error, info};
use serde::de::DeserializeOwned;
use serde_json;
use std::error::Error;

pub trait HttpRequester {
    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Box<dyn Error>>;
}

pub struct TelemetryHttpRequester;
impl HttpRequester for TelemetryHttpRequester {
    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Box<dyn Error>> {
        let response = attohttpc::get(url).send()?;
        if response.is_success() {
            let body = response.text()?;
            let parsed = serde_json::from_str::<T>(&body);

            match parsed {
                Ok(val) => return Ok(val),
                Err(e) => {
                    error!("Error parsing JSON: {e}");
                    //		    error!("Body: {}", body);
                    return Err(Box::new(e));
                }
            }
        } else {
            let message = format!(
                "Request Error: Status:{:?}, Body: {:?}",
                response.status(),
                response.text()
            );
            error!("{message}");
            Err(anyhow!(message).into())
        }
    }
}
