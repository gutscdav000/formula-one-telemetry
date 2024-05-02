#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;   
    use formula_one_telemetry::types::driver::*;
    use formula_one_telemetry::types::position::Position;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_position_request() {

	let expected_position = vec![
	    Position {
		date: "2023-08-25T10:15:06.242000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9142
	    },
	    Position {
		date: "2023-08-25T10:33:30.503000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9142 },
	    Position {
		date: "2023-08-25T10:36:38.438000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9142
	    },
	    Position {
		date: "2023-08-25T10:39:28.794000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9142
	    },
	    Position {
		date: "2023-08-25T11:22:07.295000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9142
	    },
	    Position {
		date: "2023-08-25T13:45:05.672000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9143
	    },
	    Position {
		date: "2023-08-25T14:08:59.818000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9143
	    },
	    Position {
		date: "2023-08-26T09:15:03.648000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9144
	    },
	    Position {
		date: "2023-08-26T09:52:36.685000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9144
	    },
	    Position {
		date: "2023-08-26T10:24:29.116000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9144
	    },
	    Position {
		date: "2023-08-26T12:48:11.003000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:01:36.916000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:08:06.458000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:12:28.330000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:25:12.025000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:26:57.555000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:28:19.351000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:29:40.053000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:32:33.832000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:40:46.108000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T13:49:32.399000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-26T14:25:07.812000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9145
	    },
	    Position {
		date: "2023-08-27T12:01:03.352000+00:00".to_string().to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9149

	    },
	    Position {
		date: "2023-08-27T13:22:23.234000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1217,
		position: 1,
		session_key: 9149
	    }
	];
	let driver_number = get_driver_number(&DriverName::MaxVerstappen);
	let position: Option<Vec<Position>> = API.get_position(1217, &driver_number,Some(1)); 
	println!("Position: {:?}", position);
	
	position.map_or_else(
	    || panic!("Test failed, no data received"),
	    |p| assert_eq!(p, expected_position)
	)
    }

    #[test]
    fn failed_request_position() {
	let driver_number = get_driver_number(&DriverName::MaxVerstappen);
	let position: Option<Vec<Position>> = API.get_position(999999, &DriverNumber::new(150),Some(1)); 
	assert_eq!(position, None)
    }
}
