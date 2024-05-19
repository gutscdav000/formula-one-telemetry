use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::team_radio::TeamRadio;
use async_trait::async_trait;
use log::{debug, error, info};
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
    async fn laps_sync(&self, session_key: u32, driver_number: &DriverNumber, lap: u32);
}

pub struct EventSyncImpl<'a> {
    pub api: &'a CarDataApiImpl<'a>,
    pub redis: &'a RedisImpl,
}

//TODO: return a result so we can propagate errors
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
            let maybe_car_data = self.api.get_car_data(session_key, driver_number, speed);
            if let Some(car_data) = maybe_car_data.clone() {
                if let Err(car_data) = self
                    .redis
                    .set_json::<Vec<CarData>>("car_data", car_data.clone())
                    .await
                {
                    error!("could not write car_data: {:?}", car_data);
                } else {
                    info!("car_data synced");
                }
            }
            let data_len = maybe_car_data.map_or(0, |vec| vec.len());
            info!("# requests: {counter}, data len: {data_len}");
            debug!("car_data upserted");
            counter = counter + 1;
        }
    }

    async fn intervals_sync(&self, session_key: u32, maybe_interval: Option<f32>) {
        let mut time_interval = time::interval(Duration::from_secs(5));
        loop {
            time_interval.tick().await;
            let maybe_intervals = self.api.get_intervals(session_key, maybe_interval);
            info!("maybe intervals: {:?}", maybe_intervals);
            //TODO: this doesn't error or tell you when no intervals are found
            if let Some(intervals) = maybe_intervals.clone() {
                info!("test 1");
                if let Err(intervals) = self
                    .redis
                    .set_json::<Vec<Interval>>("intervals", intervals.clone())
                    .await
                {
                    error!("could not Redis write intervals: {intervals}");
                } else {
                    info!("intervals synced");
                }
            }
        }
    }

    async fn team_radio_sync(&self, session_key: u32, driver_number: Option<DriverNumber>) {
        let mut time_interval = time::interval(Duration::from_secs(30));
        loop {
            time_interval.tick().await;
            let maybe_team_radio = self.api.get_team_radio(session_key, driver_number);
            if let Some(team_radio) = maybe_team_radio.clone() {
                if let Err(team_radio) = self
                    .redis
                    .set_json::<Vec<TeamRadio>>("team_radio", team_radio.clone())
                    .await
                {
                    error!("could not Redis write team_radio: {:?}", team_radio);
                } else {
                    info!("team radio synced");
                }
            }
        }
    }

    async fn laps_sync(&self, session_key: u32, driver_number: &DriverNumber, lap: u32) {
        let mut time_interval = time::interval(Duration::from_secs(120));
        loop {
            time_interval.tick().await;
            let maybe_laps = self.api.get_lap(session_key, driver_number, lap);
            if let Some(laps) = maybe_laps.clone() {
                if let Err(laps) = self.redis.set_json::<Vec<Lap>>("laps", laps.clone()).await {
                    error!("could not write laps: {laps}");
                } else {
                    info!("laps synced");
                }
            }
        }
    }
}
