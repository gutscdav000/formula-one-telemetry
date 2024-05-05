#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::weather::Weather;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };
	
    #[test]
    fn successful_weather_request() {
	let expected_weather = vec![
	    Weather {
		air_temperature: 27.8,
		date: "2023-05-07T18:42:25.233000+00:00".to_string(),
		humidity: 58.0,
		meeting_key: 1208,
		pressure: 1018.7,
		rainfall: 0,
		session_key: 9078,
		track_temperature: 52.5,
		wind_direction: 136,
		wind_speed: 2.4
	    }
	];

	    let weather: Option<Vec<Weather>> = API.get_weather(1208, Some(130), Some(52));
	println!("Weather: {:?}", weather);
	weather.map_or_else(
	    || panic!("Test failed, no data received"),
	    |w| assert_eq!(w, expected_weather)
	)
    }

        #[test]
    fn failed_request_weather() {
	let weather: Option<Vec<Weather>> = API.get_weather(999999, Some(130), Some(52));
	assert_eq!(weather, None)
    }
}
