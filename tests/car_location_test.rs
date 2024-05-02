#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::driver::*;
    use formula_one_telemetry::types::car_location::CarLocation;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_drivers_request() {
	let driver_number = get_driver_number(&DriverName::MaxVerstappen);
	let expected_car_locations = vec![
	    CarLocation {
		date: "2023-09-16T13:03:35.292000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1219,
		session_key: 9161,
		x: -8596,
		y: 1978,
		z: 193
	    },
	    CarLocation {
		date: "2023-09-16T13:03:35.752000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1219,
		session_key: 9161,
		x: -8708,
		y: 1846,
		z: 193
	    }
	];

	let car_loc_driver = get_driver_number(&DriverName::OscarPiastri);
	let car_locations: Option<Vec<CarLocation>> = API.get_car_location(9161, &driver_number, &"2023-09-16T13:03:35.200", &"2023-09-16T13:03:35.800");

	println!("CarLocations: {:?}", car_locations);
	car_locations.map_or_else(
	    || panic!("Test failed, no data received"),
	    |l| assert_eq!(l, expected_car_locations)
	)
    }

        #[test]
    fn failed_request_sessions() {
	let car_locations: Option<Vec<CarLocation>> = API.get_car_location(9140, &DriverNumber::new(150), &"2023-09-16T13:03:35.200", &"2023-09-16T13:03:35.800");
	assert_eq!(car_locations, None)
    }

}
