use attohttpc::{self};
use anyhow::anyhow;
use serde_json;
use serde::de::DeserializeOwned;
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
		    println!("Error parsing JSON: {}", e);
		    return Err(Box::new(e));
		}
	    }
        } else {
	    let status = response.status();
	    let message = format!("Request Error: {:?}", status);
	    println!("{}", message);
            Err(anyhow!(message).into())
        }
    }
}
