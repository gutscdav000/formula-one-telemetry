use std::vec::Vec;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::http_requester::HttpRequester;
use crate::types::driver::*;
use crate::types::car_data::CarData;
use crate::types::interval::Interval;
use crate::types::session::Session;


pub trait CarDataApi {
    fn get_session(&self, country_name: &str, session_name: &str, year: u32) -> Option<Vec<Session>>;
    fn get_drivers(&self, session_key: u32, driver_number: &DriverNumber) -> Option<Vec<Driver>>;
    fn get_car_data(&self, session_key: u32, driver_number: &DriverNumber, speed: Option<u32>) -> Option<Vec<CarData>>;
    fn get_intervals(&self, session_key: u32, maybe_interval: Option<f32>) -> Option<Vec<Interval>>;
}

pub struct CarDataApiImpl<'a> {
    pub http_requester: &'a TelemetryHttpRequester,
    pub uri: &'a str,
}

impl CarDataApi for CarDataApiImpl<'_> {
        fn get_session(&self, country_name: &str, session_name: &str, year: u32) -> Option<Vec<Session>> {
	let request_url = self.uri.to_owned() + &format!("/v1/sessions?country_name={country_name}&session_name={session_name}&year={year}");
	match self.http_requester.get::<Vec<Session>>(&request_url) {
	    Ok(sessions) if sessions.is_empty() => None,
	    Ok(sessions) => Some(sessions),
	    Err(_) => None,
	}
    }

    fn get_drivers(&self, session_key: u32, driver_number: &DriverNumber) -> Option<Vec<Driver>> {
	let request_url = self.uri.to_owned() + &format!("/v1/drivers?driver_number={}&session_key={}", driver_number, session_key);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<Driver>>(&request_url) {
	    Ok(drivers) if drivers.is_empty() => None,
	    Ok(drivers) => Some(drivers),
	    Err(_) => None,
	}
    }

    fn get_car_data(&self, session_key: u32, driver_number: &DriverNumber, speed: Option<u32>) -> Option<Vec<CarData>> {
	let speed = speed.map_or_else(|| "".to_string(), |s| format!("&speed>={}", s));
	let request_url = self.uri.to_owned() + &format!("/v1/car_data?driver_number={}&session_key={}{}", driver_number, session_key, speed);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<CarData>>(&request_url) {
	    Ok(car_data) if car_data.is_empty() => None,
	    Ok(car_data) => Some(car_data),
	    Err(_) => None,
	}
    }

    fn get_intervals(&self, session_key: u32, maybe_interval: Option<f32>) -> Option<Vec<Interval>> {
	let interval_query_param = maybe_interval.map_or_else(|| "".to_string(), |i| format!("&interval<{}", i));
	let request_url = self.uri.to_owned() + &format!("/v1/intervals?session_key={}{}", session_key, interval_query_param);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<Interval>>(&request_url) {
	    Ok(interval) if interval.is_empty() => None,
	    Ok(interval) => Some(interval),
	    Err(_) => None,
	}
    }
}
