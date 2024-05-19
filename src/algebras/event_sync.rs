use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use async_trait::async_trait;
use log::{debug, info};
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
    pub redis: &'a RedisImpl,
}

//TODO: return a result so we can propagate errors
//   | |_______________^ the `?` operator cannot be applied to type `()`
#[async_trait]
impl EventSync for EventSyncImpl<'_> {
    async fn car_data_sync(
        &self,
        session_key: u32,
        driver_number: Option<DriverNumber>,
        speed: Option<u32>,
    ) {
        let mut counter = 0;
        loop {
            time::interval(Duration::from_secs(2)).tick().await;
            //TODO: change logging to debug, add error logging
            debug!("getting car data");
            let car_data = self.api.get_car_data(session_key, driver_number, speed);
            debug!("upserting car_data to redis");
            let _ = car_data.clone().map(|cd| async move {
                self.redis
                    .set_json::<Vec<CarData>>("car_data", cd.clone())
                    .await
            });
            let data_len = car_data.map_or(0, |vec| vec.len());
            info!("# requests: {counter}, data len: {data_len}");
            // println!("date, brake, n_gear, rpn, speed, drs");
            // let _ = car_data
            //     .expect("car data not found")
            //     .into_iter()
            //     .for_each(|cd: CarData| {
            //         println!(
            //             "{}, {}, {}, {}, {}, {}",
            //             cd.date, cd.brake, cd.n_gear, cd.rpm, cd.speed, cd.drs
            //         )
            //     });
            debug!("car_data upserted");
            counter = counter + 1;
        }
    }
}
