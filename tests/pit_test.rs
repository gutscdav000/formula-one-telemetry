#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::pit::Pit;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl {
        http_requester: &HTTP_REQUESTER,
        uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_pit_request() {
        let expected_pit = vec![Pit {
            date: "2023-09-15T09:38:23.038000+00:00".to_string(),
            driver_number: 63,
            lap_number: 5,
            meeting_key: 1219,
            pit_duration: Some(24.5),
            session_key: 9158,
        }];

        let pit: Option<Vec<Pit>> = API.get_pit(9158, Some(25));
        println!("Pit: {:?}", pit);

        pit.map_or_else(
            || panic!("Test failed, no data received"),
            |p| assert_eq!(p, expected_pit),
        )
    }

    #[test]
    fn failed_request_pit() {
        let pit: Option<Vec<Pit>> = API.get_pit(9158, Some(1));
        assert_eq!(pit, None)
    }
}
