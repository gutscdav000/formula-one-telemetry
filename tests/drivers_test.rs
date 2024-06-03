#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::driver::*;
    use formula_one_telemetry::types::session::Session;

    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl {
        http_requester: &HTTP_REQUESTER,
        uri: "https://api.openf1.org",
    };

    #[test]
    fn successful_drivers_request() {
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
        let driver_number = DriverNumber::get_driver_number(&DriverName::MaxVerstappen);
        let expected_driver = vec![Driver {
            driver_number: 1,
            broadcast_name: "M VERSTAPPEN".to_string(),
            full_name: "Max VERSTAPPEN".to_string(),
            name_acronym: "VER".to_string(),
            session_key: 9140,
            meeting_key: Some(1216),
            team_name: None,
            first_name: None,
            team_colour: None,
            country_code: None,
            last_name: None,
            headshot_url: None,
        }];

        let drivers: Option<Vec<Driver>> = API.get_drivers(session.session_key, &driver_number);
        println!("Drivers: {:?}", drivers);
        drivers.map_or_else(
            || panic!("Test failed, no data received"),
            |d| assert_eq!(d, expected_driver),
        )
    }

    #[test]
    fn failed_request_driver() {
        let drivers: Option<Vec<Driver>> = API.get_drivers(999999, &DriverNumber::new(950));
        assert_eq!(drivers, None)
    }

    #[test]
    fn serialize_driver_number() {
        let driver_number = DriverNumber::get_driver_number(&DriverName::MaxVerstappen);
        let serialized = serde_json::to_string(&driver_number).unwrap();
        assert_eq!(serialized, "1");

        let deserialized: DriverNumber = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, driver_number);
    }
}
