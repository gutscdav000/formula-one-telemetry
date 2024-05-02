#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::session::Session;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };
	
    #[test]
    fn successful_sessions_request() {

	let expected_sessions: Vec<Session> =
	    vec![
		Session {
		    circuit_key: 7,
		    circuit_short_name: "Spa-Francorchamps".to_string(),
		    country_code: "BEL".to_string(),
		    country_key: 16,
		    country_name: "Belgium".to_string(),
		    date_end: "2023-07-29T15:35:00+00:00".to_string(),
		    date_start: "2023-07-29T15:05:00+00:00".to_string(),
		    gmt_offset: "02:00:00".to_string(),
		    location: "Spa-Francorchamps".to_string(),
		    meeting_key: 1216,
		    session_key: 9140,
		    session_name: "Sprint".to_string(),
		    session_type: "Race".to_string(),
		    year: 2023 }
	    ];
	
	let sessions: Option<Vec<Session>> = API.get_session(&"Belgium".to_string(), &"Sprint".to_string(), 2023);
	println!("Sessions: {:?}", sessions);
	sessions.map_or_else(
	    || panic!("Test failed, no data received"),
	    |s| assert_eq!(s, expected_sessions)
	)
    }

    #[test]
    fn failed_request_sessions() {
	let sessions: Option<Vec<Session>> = API.get_session(&"fake".to_string(), &"fake".to_string(), 9080);
	assert_eq!(sessions, None)
    }
}
