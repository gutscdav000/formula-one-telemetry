#[cfg(test)]
mod test {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::driver::*;
    use formula_one_telemetry::types::team_radio::TeamRadio;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl {
        http_requester: &HTTP_REQUESTER,
        uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_sessions_request() {
        let driver_number = get_driver_number(&DriverName::MaxVerstappen);
        let expected_team_radio = vec![
	    TeamRadio {
		date: "2023-09-15T09:39:02.584000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1219,
		recording_url: "https://livetiming.formula1.com/static/2023/2023-09-17_Singapore_Grand_Prix/2023-09-15_Practice_1/TeamRadio/MAXVER01_1_20230915_103928.mp3".to_string(),
		session_key: 9158
	    },
	    TeamRadio {
		date: "2023-09-15T09:45:45.265000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1219,
		recording_url: "https://livetiming.formula1.com/static/2023/2023-09-17_Singapore_Grand_Prix/2023-09-15_Practice_1/TeamRadio/MAXVER01_1_20230915_104611.mp3".to_string(),
		session_key: 9158
	    },
	    TeamRadio {
		date: "2023-09-15T10:04:14.239000+00:00".to_string(),
		driver_number: 1,
		meeting_key: 1219,
		recording_url: "https://livetiming.formula1.com/static/2023/2023-09-17_Singapore_Grand_Prix/2023-09-15_Practice_1/TeamRadio/MAXVER01_1_20230915_110349.mp3".to_string(),
		session_key: 9158
	    }
	];
        let team_radio: Option<Vec<TeamRadio>> = API.get_team_radio(9158, Some(driver_number));
        println!("TeamRadio: {:?}", team_radio);
        team_radio.map_or_else(
            || panic!("Test failed, no data received"),
            |tr| assert_eq!(tr, expected_team_radio),
        )
    }

    #[test]
    fn failed_request_team_radio() {
        let team_radio: Option<Vec<TeamRadio>> = API.get_team_radio(99999, None);
        assert_eq!(team_radio, None)
    }
}
