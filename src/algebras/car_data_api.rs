use std::vec::Vec;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::http_requester::HttpRequester;
use crate::types::driver::*;
use crate::types::car_data::CarData;
use crate::types::car_location::CarLocation;
use crate::types::flag::Flag;
use crate::types::interval::Interval;
use crate::types::lap::Lap;
use crate::types::meeting::Meeting;
use crate::types::pit::Pit;
use crate::types::race_controls::*;
use crate::types::position::Position;
use crate::types::session::Session;


pub trait CarDataApi {
    fn get_session(&self, country_name: &str, session_name: &str, year: u32) -> Option<Vec<Session>>;
    fn get_drivers(&self, session_key: u32, driver_number: &DriverNumber) -> Option<Vec<Driver>>;
    fn get_car_data(&self, session_key: u32, driver_number: &DriverNumber, speed: Option<u32>) -> Option<Vec<CarData>>;
    fn get_intervals(&self, session_key: u32, maybe_interval: Option<f32>) -> Option<Vec<Interval>>;
    fn get_lap(&self, session_key: u32, driver_number: &DriverNumber, lap: u32) -> Option<Vec<Lap>>;
    fn get_car_location(&self, session_key: u32, driver_number: &DriverNumber, start_time: &str, end_time: &str) -> Option<Vec<CarLocation>>;
    fn get_meeting(&self, year: u32, country: &str) -> Option<Vec<Meeting>>;
    fn get_pit(&self, session_key: u32, pit_duration: Option<u32>) -> Option<Vec<Pit>>;
    fn get_position(&self, meeting_key: u32, driver_number: &DriverNumber, position: Option<u32>) -> Option<Vec<Position>>;
    fn get_race_control(&self, category: Option<Category>, flag: Option<Flag>, driver_number: Option<DriverNumber>, start_date: Option<String>, end_date: Option<String>) -> Option<Vec<RaceControl>>;
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

    fn get_lap(&self, session_key: u32, driver_number: &DriverNumber, lap: u32) -> Option<Vec<Lap>> {
	let request_url = self.uri.to_owned() + &format!("/v1/laps?session_key={}&driver_number={}&lap_number={}", session_key, driver_number, lap);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<Lap>>(&request_url) {
	    Ok(laps) if laps.is_empty() => None,
	    Ok(laps) => Some(laps),
	    Err(_) => None,
	}
    }

    fn get_car_location(&self, session_key: u32, driver_number: &DriverNumber, start_time: &str, end_time: &str) -> Option<Vec<CarLocation>> {
	let request_url = self.uri.to_owned() + &format!("/v1/location?session_key={}&driver_number={}&date>{}&date<{}", session_key, driver_number, start_time, end_time);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<CarLocation>>(&request_url) {
	    Ok(locations) if locations.is_empty() => None,
	    Ok(locations) => Some(locations),
	    Err(_) => None,
	}
    }

    fn get_meeting(&self, year: u32, country: &str) -> Option<Vec<Meeting>> {
	let request_url = self.uri.to_owned() + &format!("/v1/meetings?year={}&country_name={}", year, country);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<Meeting>>(&request_url) {
	    Ok(meeting) if meeting.is_empty() => None,
	    Ok(meeting) => Some(meeting),
	    Err(_) => None,
	}
    }

    fn get_pit(&self, session_key: u32, pit_duration: Option<u32>) -> Option<Vec<Pit>> {
	let pit_duration_str = pit_duration.map_or_else(|| "".to_string(), |p| format!("&pit_duration<{}", &p));
	let request_url = self.uri.to_owned() + &format!("/v1/pit?session_key={}{}", session_key, pit_duration_str);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<Pit>>(&request_url) {
	    Ok(pit) if pit.is_empty() => None,
	    Ok(pit) => Some(pit),
	    Err(_) => None,
	}
    }

    fn get_position(&self, meeting_key: u32, driver_number: &DriverNumber, position: Option<u32>) -> Option<Vec<Position>> {
	let position_str = position.map_or_else(|| "".to_string(), |p| format!("&position<={}", p));
	let request_url = self.uri.to_owned() + &format!("/v1/position?meeting_key={}&driver_number={}{}", meeting_key, driver_number, position_str);
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<Position>>(&request_url) {
	    Ok(position) if position.is_empty() => None,
	    Ok(position) => Some(position),
	    Err(_) => None,
	}
    }

    fn get_race_control(&self, category: Option<Category>, flag: Option<Flag>, driver_number: Option<DriverNumber>, start_date: Option<String>, end_date: Option<String>) -> Option<Vec<RaceControl>> {
	let params = build_query_params(category, flag, driver_number, start_date, end_date);
	let request_url = self.uri.to_owned() + &"/v1/race_control" + &params;
	println!("{:?}", request_url);
	match self.http_requester.get::<Vec<RaceControl>>(&request_url) {
	    Ok(race_control) if race_control.is_empty() => None,
	    Ok(race_control) => Some(race_control),
	    Err(_) => None,
	}
    }
}

        fn build_query_params(category: Option<Category>, flag: Option<Flag>, driver_number: Option<DriverNumber>, start_date: Option<String>, end_date: Option<String>) -> String {
		let category_str = category.map_or_else(|| "".to_string(), |c| format!("&category={:?}", c));
	let flag_str = flag.map_or_else(|| "".to_string(), |f| format!("&flag={}", f));
	let driver_num_str = driver_number.map_or_else(|| "".to_string(), |d| format!("&driver_number={}", d));
	let start_date_str = start_date.map_or_else(|| "".to_string(), |d| format!("&date>={}", d));
	let end_date_str = end_date.map_or_else(|| "".to_string(), |d| format!("&date<{}", d));
	let s = category_str + &flag_str + &driver_num_str + &start_date_str + &end_date_str;

	if!s.is_empty() && s.starts_with('&') {
	    ("?".to_owned() + &s[1..]).to_string()
	} else {
	    s
	}
    }
