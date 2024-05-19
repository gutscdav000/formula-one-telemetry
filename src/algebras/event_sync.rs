use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use crate::types::interval::Interval;
use crate::types::team_radio::TeamRadio;
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
    async fn intervals_sync(&self, session_key: u32, maybe_interval: Option<f32>);
    async fn team_radio_sync(&self, session_key: u32, driver_number: Option<DriverNumber>);
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
        let mut time_interval = time::interval(Duration::from_secs(2));
        let mut counter = 0;
        loop {
            time_interval.tick().await;
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

    async fn intervals_sync(&self, session_key: u32, maybe_interval: Option<f32>) {
        let mut time_interval = time::interval(Duration::from_secs(60));
        loop {
            time_interval.tick().await;
            //TODO: change logging to debug, add error logging
            debug!("getting intervals");
            let intervals = self.api.get_intervals(session_key, maybe_interval);
            debug!("upserting intervals to redis");
            let _ = intervals.clone().map(|i| async move {
                self.redis
                    .set_json::<Vec<Interval>>("interval", i.clone())
                    .await
            });
            info!("intervals synced");
        }
    }

    async fn team_radio_sync(&self, session_key: u32, driver_number: Option<DriverNumber>) {
        let mut time_interval = time::interval(Duration::from_secs(30));
        loop {
            time_interval.tick().await;
            //TODO: change logging to debug, add error logging
            debug!("getting team radio");
            let team_radio = self.api.get_team_radio(session_key, driver_number);
            debug!("upserting team radio to redis");
            let _ = team_radio.clone().map(|tr| async move {
                self.redis
                    .set_json::<Vec<TeamRadio>>("team_radio", tr.clone())
                    .await
            });
            info!("team radio synced");
        }
    }
}
