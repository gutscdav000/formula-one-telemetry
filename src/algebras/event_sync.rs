use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::channel_queue::*;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use crate::types::event_sync::EventSyncConfig;
use crate::types::events::*;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::pit::Pit;
use crate::types::position::Position;
use crate::types::stint::Stint;
use crate::types::team_radio::TeamRadio;
use async_trait::async_trait;
use log::error;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
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
    async fn stints_sync(self: Arc<Self>, session_key: u32, tyre_age: Option<u32>);
    async fn run_sync(
        self: Arc<Self>,
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
    pub delay_config: &'a EventSyncConfig,
    pub tx: Arc<dyn ChannelQueue>,
}

#[async_trait]
impl EventSync for EventSyncImpl<'_> {
    async fn car_data_sync(
        &self,
        session_key: u32,
        driver_number: Option<DriverNumber>,
        speed: Option<u32>,
    ) {
        let mut time_interval = time::interval(Duration::from_secs(
            self.delay_config.car_data_duration_secs,
        ));
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
            //TODO: this was for sending Events
            // if let Err(e) = self.tx.send(Events::CarData) {
            //     error!("failed to send Events message: {e}");
            // }
            let v: Vec<dyn EventData> =
                maybe_car_data.unwrap_or(Vec::<CarData>::new()) as Vec<dyn EventData>;
            if !v.is_empty() {
                if let Err(e) = self.tx.send(Message::<dyn EventData> { value: v }) {
                    error!("failed to send Events message: {e}");
                }
            }
        }
    }

    async fn intervals_sync(&self, session_key: u32, maybe_interval: Option<f32>) {
        let mut time_interval = time::interval(Duration::from_secs(
            self.delay_config.interval_duration_secs,
        ));
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
        let mut time_interval = time::interval(Duration::from_secs(
            self.delay_config.team_radio_duration_secs,
        ));
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
        let mut time_interval =
            time::interval(Duration::from_secs(self.delay_config.laps_duration_secs));
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
        let mut time_interval =
            time::interval(Duration::from_secs(self.delay_config.pit_duration_secs));
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
        let mut time_interval = time::interval(Duration::from_secs(
            self.delay_config.position_duration_secs,
        ));
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

    async fn stints_sync(self: Arc<Self>, session_key: u32, tyre_age: Option<u32>) {
        let mut time_interval =
            time::interval(Duration::from_secs(self.delay_config.stints_duration_secs));
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
        self: Arc<Self>,
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
        let self_clone = Arc::clone(&self);
        tokio_scoped::scope(|scope| {
            scope.spawn(async move {
                tokio::select! {
                        _ = self_clone.car_data_sync(session_key, Some(driver_number), speed) => {},
                        _ = self_clone.intervals_sync(session_key, maybe_interval) => {},
                // We want to sync all team radio, not by driver
                        _ = self_clone.team_radio_sync(session_key, None) => {},
                //TODO: Research required: lap could cause issues, depending on value provided.
                        _ = self_clone.laps_sync(session_key, &driver_number, lap) => {},
                        _ = self_clone.pit_sync(session_key, pit_duration) => {},
                        _ = self_clone.position_sync(meeting_key, &driver_number, position) => {},
                        _ = self.stints_sync(session_key, tyre_age) => {},
                    }
            });
        });
    }
}
