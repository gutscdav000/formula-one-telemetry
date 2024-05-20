use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::pit::Pit;
use crate::types::position::Position;
use crate::types::stint::Stint;
use crate::types::team_radio::TeamRadio;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde::Serialize;
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
    async fn pit_sync(&self, session_key: u32, pit_duration: Option<u32>);
    async fn position_sync(
        &self,
        meeting_key: u32,
        driver_number: &DriverNumber,
        position: Option<u32>,
    );
    async fn stints_sync(&self, session_key: u32, tyre_age: Option<u32>);
    async fn run_sync(
        &self,
        session_key: u32,
        meeting_key: u32,
        speed: Option<u32>,
        maybe_interval: Option<f32>,
        driver_number: DriverNumber,
        lap: u32,
        pit_duration: Option<u32>,
        position: Option<u32>,
        tyre_age: Option<u32>,
    );
}

pub struct EventSyncImpl<'a> {
    pub api: &'a CarDataApiImpl<'a>,
    pub redis: &'a RedisImpl,
}

//TODO: return a result so we can propagate errors
#[async_trait]
impl EventSync for EventSyncImpl<'_> {
    //TODO: clean this up once we're finished
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
            let _ = self
                .redis
                .redis_fire_and_forget::<CarData>(
                    maybe_car_data.clone(),
                    String::from(format!(
                        "car_data:{}",
                        driver_number.unwrap_or(DriverNumber::new(0)) //this state should be unrepresntable, but still an anti pattern
                    )),
                )
                .await;
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
            let _ = self
                .redis
                .redis_fire_and_forget::<Interval>(
                    maybe_intervals.clone(),
                    String::from("intervals"),
                )
                .await;
        }
    }

    async fn team_radio_sync(&self, session_key: u32, driver_number: Option<DriverNumber>) {
        let mut time_interval = time::interval(Duration::from_secs(30));
        loop {
            time_interval.tick().await;
            let maybe_team_radio = self.api.get_team_radio(session_key, driver_number);
            let _ = self
                .redis
                .redis_fire_and_forget::<TeamRadio>(
                    maybe_team_radio.clone(),
                    String::from("team_radio"),
                )
                .await;
        }
    }

    async fn laps_sync(&self, session_key: u32, driver_number: &DriverNumber, lap: u32) {
        let mut time_interval = time::interval(Duration::from_secs(120));
        loop {
            time_interval.tick().await;
            let maybe_laps = self.api.get_lap(session_key, driver_number, lap);
            let _ = self
                .redis
                .redis_fire_and_forget::<Lap>(maybe_laps.clone(), String::from("laps"))
                .await;
        }
    }

    async fn pit_sync(&self, session_key: u32, pit_duration: Option<u32>) {
        let mut time_interval = time::interval(Duration::from_secs(120));
        loop {
            time_interval.tick().await;
            let maybe_pits = self.api.get_pit(session_key, pit_duration);
            let _ = self
                .redis
                .redis_fire_and_forget::<Pit>(maybe_pits.clone(), String::from("pits"))
                .await;
        }
    }

    async fn position_sync(
        &self,
        meeting_key: u32,
        driver_number: &DriverNumber,
        position: Option<u32>,
    ) {
        let mut time_interval = time::interval(Duration::from_secs(5));
        loop {
            time_interval.tick().await;
            let maybe_position = self.api.get_position(meeting_key, driver_number, position);
            let _ = self
                .redis
                .redis_fire_and_forget::<Position>(
                    maybe_position.clone(),
                    String::from(format!("position:{}", driver_number,)),
                )
                .await;
        }
    }

    async fn stints_sync(&self, session_key: u32, tyre_age: Option<u32>) {
        let mut time_interval = time::interval(Duration::from_secs(120));
        loop {
            time_interval.tick().await;
            let maybe_stints = self.api.get_stints(session_key, tyre_age);
            let _ = self
                .redis
                .redis_fire_and_forget::<Stint>(maybe_stints.clone(), String::from("stints"))
                .await;
        }
    }

    async fn run_sync(
        &self,
        session_key: u32,
        meeting_key: u32,
        speed: Option<u32>,
        maybe_interval: Option<f32>,
        driver_number: DriverNumber,
        lap: u32,
        pit_duration: Option<u32>,
        position: Option<u32>,
        tyre_age: Option<u32>,
    ) {
        tokio_scoped::scope(|scope| {
            scope.spawn(async move {
                tokio::select! {
                        _ = self.car_data_sync(session_key, Some(driver_number), speed) => {},
                        _ = self.intervals_sync(session_key, maybe_interval) => {},
                // We want to sync all team radio, not by driver
                        _ = self.team_radio_sync(session_key, None) => {},
                //TODO: Research required: lap could cause issues, depending on value provided.
                        _ = self.laps_sync(session_key, &driver_number, lap) => {},
                        _ = self.pit_sync(session_key, pit_duration) => {},
                        _ = self.position_sync(meeting_key, &driver_number, position) => {},
                        _ = self.stints_sync(session_key, tyre_age) => {},
                    }
            });
        });
    }
}
