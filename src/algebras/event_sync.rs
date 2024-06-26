use crate::algebras::car_data_api::CarDataApi;
use crate::algebras::car_data_api::CarDataApiImpl;
use crate::algebras::channel_queue::*;
use crate::algebras::redis::Redis;
use crate::algebras::redis::RedisImpl;
use crate::types::car_data::CarData;
use crate::types::driver::*;
use crate::types::event::Event;
use crate::types::event_sync::EventSyncConfig;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::pit::Pit;
use crate::types::position::Position;
use crate::types::stint::Stint;
use crate::types::team_radio::TeamRadio;
use async_trait::async_trait;
use log::info;
use std::sync::Arc;
use tokio::time::{self, Duration};

#[async_trait]
pub trait EventSync {
    async fn car_data_upsert(
        &self,
        session_key: u32,
        driver_number: &DriverNumber,
        speed: Option<u32>,
    );
    async fn car_data_sync(&self, session_key: u32, speed: Option<u32>);
    async fn intervals_sync(&self, session_key: u32, maybe_interval: Option<f32>);
    async fn team_radio_sync(&self, session_key: u32, driver_number: Option<DriverNumber>);
    async fn laps_upsert(&self, session_key: u32, driver_number: &DriverNumber, lap: u32);
    async fn laps_sync(&self, session_key: u32);
    async fn pit_sync(&self, session_key: u32, pit_duration: Option<u32>);
    async fn position_upsert(
        &self,
        meeting_key: u32,
        driver_number: &DriverNumber,
        position: Option<u32>,
    );
    async fn position_sync(&self, meeting_key: u32, position: Option<u32>);
    async fn stints_sync(&self, session_key: u32, tyre_age: Option<u32>);
    async fn run_sync(
        &self,
        session_key: u32,
        meeting_key: u32,
        speed: Option<u32>,
        maybe_interval: Option<f32>,
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
    async fn car_data_upsert(
        &self,
        session_key: u32,
        driver_number: &DriverNumber,
        speed: Option<u32>,
    ) {
        let maybe_car_data = self
            .api
            .get_car_data(session_key, Some(*driver_number), speed);
        let _ = self
            .redis
            .redis_fire_and_forget::<CarData>(
                maybe_car_data.clone(),
                String::from(format!("car_data:{}", driver_number)),
            )
            .await;
    }

    async fn car_data_sync(&self, session_key: u32, speed: Option<u32>) {
        let mut time_interval = time::interval(Duration::from_secs(
            self.delay_config.car_data_duration_secs,
        ));
        loop {
            time_interval.tick().await;
            tokio_scoped::scope(|scope| {
                DriverNumber::all_variants()
                    .into_iter()
                    .for_each(|driver_number| {
                        scope.spawn(async move {
                            let _ = self
                                .car_data_upsert(session_key, &driver_number, speed)
                                .await;
                        });
                    });
            });
            info!("Car Data Synced");
            self.tx.fire_and_forget(Event::CarData);
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
            info!("Intervals Synced");
            self.tx.fire_and_forget(Event::Interval);
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
            info!("Team Radio Synced");
            self.tx.fire_and_forget(Event::TeamRadio);
        }
    }
    async fn laps_upsert(&self, session_key: u32, driver_number: &DriverNumber, lap: u32) {
        let maybe_laps = self.api.get_lap(session_key, driver_number, lap);
        let _ = self
            .redis
            .redis_fire_and_forget::<Lap>(
                maybe_laps.clone(),
                String::from(format!("laps:{}", driver_number,)),
            )
            .await;
    }
    async fn laps_sync(&self, session_key: u32) {
        let mut time_interval =
            time::interval(Duration::from_secs(self.delay_config.laps_duration_secs));
        let mut lap_number: u32 = 1;
        loop {
            time_interval.tick().await;
            tokio_scoped::scope(|scope| {
                DriverNumber::all_variants()
                    .into_iter()
                    .for_each(|driver_number| {
                        scope.spawn(async move {
                            let _ = self.laps_upsert(session_key, &driver_number, lap_number);
                        });
                    });
            });
            info!("Laps Synced");
            self.tx.fire_and_forget(Event::Lap);
            lap_number += 1;
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
            info!("Pits Synced");
            self.tx.fire_and_forget(Event::Pit);
        }
    }
    async fn position_upsert(
        &self,
        meeting_key: u32,
        driver_number: &DriverNumber,
        position: Option<u32>,
    ) {
        let maybe_position = self.api.get_position(meeting_key, driver_number, position);
        let _ = self
            .redis
            .redis_fire_and_forget::<Position>(
                maybe_position.clone(),
                String::from(format!("position:{}", driver_number,)),
            )
            .await;
    }
    async fn position_sync(&self, meeting_key: u32, position: Option<u32>) {
        let mut time_interval = time::interval(Duration::from_secs(
            self.delay_config.position_duration_secs,
        ));
        loop {
            time_interval.tick().await;
            tokio_scoped::scope(|scope| {
                DriverNumber::all_variants()
                    .into_iter()
                    .for_each(|driver_number| {
                        scope.spawn(async move {
                            let _ = self
                                .position_upsert(meeting_key, &driver_number, position)
                                .await;
                        });
                    });
            });
            info!("Positions Synced");
            self.tx.fire_and_forget(Event::Position);
        }
    }

    async fn stints_sync(&self, session_key: u32, tyre_age: Option<u32>) {
        let mut time_interval =
            time::interval(Duration::from_secs(self.delay_config.stints_duration_secs));
        loop {
            time_interval.tick().await;
            let maybe_stints = self.api.get_stints(session_key, tyre_age);
            let _ = self
                .redis
                .redis_fire_and_forget::<Stint>(maybe_stints.clone(), String::from("stints"))
                .await;
            info!("Stints Synced");
            self.tx.fire_and_forget(Event::Stint);
        }
    }

    async fn run_sync(
        &self,
        session_key: u32,
        meeting_key: u32,
        speed: Option<u32>,
        maybe_interval: Option<f32>,
        pit_duration: Option<u32>,
        position: Option<u32>,
        tyre_age: Option<u32>,
    ) {
        tokio_scoped::scope(|scope| {
            scope.spawn(async move {
                tokio::join!(
                    self.car_data_sync(session_key, speed),
                    self.intervals_sync(session_key, maybe_interval),
                    // We want to sync all team radio, not by driver
                    self.team_radio_sync(session_key, None),
                    //TODO: Research required: lap could cause issues, depending on value provided.
                    //NOTE: not using laps because incrementing lap only works if app restarted at session start only for Race
                    //self.laps_sync(session_key),
                    self.pit_sync(session_key, pit_duration),
                    self.position_sync(meeting_key, position),
                    self.stints_sync(session_key, tyre_age),
                );
            });
        });
    }
}
