#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::meeting::Meeting;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_drivers_request() {

	let expected_meeting = vec![
	    Meeting {
		circuit_key: 61,
		circuit_short_name: "Singapore".to_string(),
		country_code: "SGP".to_string(),
		country_key: 157,
		country_name: "Singapore".to_string(),
		date_start: "2023-09-15T09:30:00+00:00".to_string(),
		gmt_offset: "08:00:00".to_string(),
		location: "Marina Bay".to_string(),
		meeting_key: 1219,
		meeting_name: "Singapore Grand Prix".to_string(),
		meeting_official_name: "FORMULA 1 SINGAPORE AIRLINES SINGAPORE GRAND PRIX 2023".to_string(),
		year: 2023
	    }
	];

	let meeting: Option<Vec<Meeting>> = API.get_meeting(2023, &"Singapore");
	println!("Meeting: {:?}", meeting);
	
	meeting.map_or_else(
	    || panic!("Test failed, no data received"),
	    |m| assert_eq!(m, expected_meeting)
	)
    }

    #[test]
    fn failed_request_sessions() {
	let meeting: Option<Vec<Meeting>> = API.get_meeting(2026, &"Rajasthan");
	assert_eq!(meeting, None)
    }
}
	
