use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use async_trait::async_trait;
use tokio::time::{self, Duration};

#[async_trait]
pub trait EventSync {
    async fn car_data_sync(
        &self,
        session_key: u32,
        driver_number: Option<DriverNumber>,
        speed: Option<u32>,
    );
}

pub struct EventSyncImpl<'a> {
    pub api: &'a CarDataApiImpl<'a>,
}

#[async_trait]
impl EventSync for EventSyncImpl<'_> {
    async fn car_data_sync(
        &self,
        session_key: u32,
        driver_number: Option<DriverNumber>,
        speed: Option<u32>,
    ) {
        loop {
            time::interval(Duration::from_secs(2)).tick().await;
            let car_data = self.api.get_car_data(session_key, driver_number, speed);
            // redis upsert here...
            //try and remove this mut bull shit makes me want to kill myeself...
        }
    }
}
