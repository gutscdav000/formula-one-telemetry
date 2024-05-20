#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::driver::*;
    use formula_one_telemetry::types::lap::Lap;
    use formula_one_telemetry::types::session::Session;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl {
        http_requester: &HTTP_REQUESTER,
        uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_lap_request() {
        let session = Session {
            circuit_key: 7,
            circuit_short_name: "Spa-Francorchamps".to_string(),
            country_code: "BEL".to_string(),
            country_key: 16,
            country_name: "Belgium".to_string(),
            date_end: "2023-07-29T15:35:00".to_string(),
            date_start: "2023-07-29T15:05:00".to_string(),
            gmt_offset: "02:00:00".to_string(),
            location: "Spa-Francorchamps".to_string(),
            meeting_key: 1216,
            session_key: 9140,
            session_name: "Sprint".to_string(),
            session_type: "Race".to_string(),
            year: 2023,
        };
        let driver_number = get_driver_number(&DriverName::MaxVerstappen);

        let expected_laps = vec![Lap {
            date_start: "2023-07-29T16:07:30.848000+00:00".to_string(),
            driver_number: 1,
            duration_sector_1: Some(33.745),
            duration_sector_2: Some(55.612),
            duration_sector_3: Some(32.06),
            i1_speed: 284,
            i2_speed: 174,
            is_pit_out_lap: false,
            lap_duration: Some(121.417),
            lap_number: 8,
            meeting_key: 1216,
            session_key: 9140,
            st_speed: 288,
        }];

        let laps: Option<Vec<Lap>> = API.get_lap(session.session_key, &driver_number, 8);
        println!("Laps: {:?}", laps);
        laps.map_or_else(
            || panic!("Test failed, no data received"),
            |l| assert_eq!(l, expected_laps),
        )
    }

    #[test]
    fn failed_request_laps() {
        let laps: Option<Vec<Lap>> = API.get_lap(999999, &DriverNumber::new(950), 150);
        assert_eq!(laps, None)
    }
}
