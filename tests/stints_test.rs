#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::stint::Stint;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl {
        http_requester: &HTTP_REQUESTER,
        uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_sessions_request() {
        let expected_stints = vec![
            Stint {
                compound: "SOFT".to_string(),
                driver_number: 16,
                lap_end: 20,
                lap_start: 1,
                meeting_key: 1219,
                session_key: 9165,
                stint_number: 1,
                tyre_age_at_start: 3,
            },
            Stint {
                compound: "SOFT".to_string(),
                driver_number: 20,
                lap_end: 62,
                lap_start: 44,
                meeting_key: 1219,
                session_key: 9165,
                stint_number: 3,
                tyre_age_at_start: 3,
            },
        ];
        let stints: Option<Vec<Stint>> = API.get_stints(9165, Some(3));
        println!("Stint: {:?}", stints);

        stints.map_or_else(
            || panic!("Test failed, no data received"),
            |s| assert_eq!(s, expected_stints),
        )
    }

    #[test]
    fn failed_request_sessions() {
        let stints: Option<Vec<Stint>> = API.get_stints(999999, None);
        assert_eq!(stints, None)
    }
}
