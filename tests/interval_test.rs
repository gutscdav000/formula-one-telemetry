#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::interval::Interval;
    use formula_one_telemetry::types::session::Session;
    
    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_car_data_request() {
	let session = Session {
		circuit_key: 7,
		circuit_short_name: "Spa-Francorchamps".to_string(),
		country_code: "BEL".to_string(), country_key: 16,
		country_name: "Belgium".to_string(),
		date_end: "2023-07-29T15:35:00".to_string(),
		date_start: "2023-07-29T15:05:00".to_string(),
		gmt_offset: "02:00:00".to_string(),
		location: "Spa-Francorchamps".to_string(),
		meeting_key: 1216,
		session_key: 9140,
		session_name: "Sprint".to_string(),
		session_type: "Race".to_string(),
		year: 2023
	};
	let expected_interval = vec! [
	    Interval {
		gap_to_leader: Some(6.961),
		interval: 0.007,
		driver_number: 18,
		date: "2023-07-29T16:04:11.210000+00:00".to_string(),
		session_key: 9140,
		meeting_key: 1216
	    },
	    Interval {
		gap_to_leader: Some(14.149),
		interval: 0.008,
		driver_number: 31,
		date: "2023-07-29T16:09:50.972000+00:00".to_string(),
		session_key: 9140,
		meeting_key: 1216
	    }
	];

	let interval_filter: Option<f32> = Some(0.01);
	let interval: Option<Vec<Interval>> = API.get_intervals(session.session_key, interval_filter);
	println!("Interval: {:?}", interval);
	interval.map_or_else(
	    || panic!("Test failed, no data received"),
	    |d| assert_eq!(d, expected_interval)
	)
    }

    #[test]
    fn failed_interval_request() {
	let interval: Option<Vec<Interval>> = API.get_intervals(99999, Some(0.0001));
	assert_eq!(interval, None)
    }
}
