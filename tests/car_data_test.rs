#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::car_data::CarData;
    use formula_one_telemetry::types::driver::*;
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
	let driver_number = get_driver_number(&DriverName::MaxVerstappen);
	let expected_car_data = vec! [
	    CarData {
		brake: 0,
		date: "2023-07-29T14:40:18.279000".to_string(),
		driver_number: 1,
		drs: 1,
		meeting_key: 1216,
		n_gear: 8,
		rpm: 11225,
		session_key: 9140,
		speed: 315,
		throttle: 100
	    },
	    CarData {
		brake: 0,
		date: "2023-07-29T14:40:18.479000".to_string(),
		driver_number: 1,
		drs: 1,
		meeting_key: 1216,
		n_gear: 8,
		rpm: 11168,
		session_key: 9140,
		speed: 315,
		throttle: 100
	    }
	];
	let car_data: Option<Vec<CarData>> = API.get_car_data(session.session_key, &driver_number, Some(315));
	println!("CarData: {:?}", car_data);
	car_data.map_or_else(
	    || panic!("Test failed, no data received"),
	    |d| assert_eq!(d, expected_car_data)
	)	
    }

        #[test]
    fn failed_car_data_request() {
	let drivers: Option<Vec<CarData>> = API.get_car_data(99999, &DriverNumber::new(950), Some(315));
	assert_eq!(drivers, None)
    }
}
