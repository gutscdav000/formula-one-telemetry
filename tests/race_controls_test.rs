#[cfg(test)]
mod tests {
    use formula_one_telemetry::algebras::car_data_api::CarDataApi;
    use formula_one_telemetry::algebras::car_data_api::CarDataApiImpl;
    use formula_one_telemetry::algebras::http_requester::TelemetryHttpRequester;
    use formula_one_telemetry::types::driver::*;
    use formula_one_telemetry::types::flag::*;
    use formula_one_telemetry::types::race_controls::*;
    use serde_json;
    
    const HTTP_REQUESTER: TelemetryHttpRequester = TelemetryHttpRequester;
    const API: CarDataApiImpl = CarDataApiImpl{
	http_requester: &HTTP_REQUESTER,
	uri: "https://api.openf1.org",
    };
	
    #[test]
    fn successful_race_controls_request() {
	let driver_number = get_driver_number(&DriverName::MaxVerstappen);
	let expected_race_control = vec![
	    RaceControl {
		category: Category::Flag,
		date: "2023-06-04T14:21:01+00:00".to_string(),
		driver_number: driver_number,
		flag: Flag::BlackAndWhite,
		lap_number: 59,
		meeting_key: 1211,
		message: "BLACK AND WHITE FLAG FOR CAR 1 (VER) - TRACK LIMITS".to_string(),
		scope: "Driver".to_string(),
		sector: None,
		session_key: 9102
	    }
	];

	let race_control: Option<Vec<RaceControl>> = API.get_race_control(Some(Category::Flag), Some(Flag::BlackAndWhite), Some(driver_number), None, None);
	race_control.map_or_else(
	    || panic!("Test failed, no data received"),
	    |rc| assert_eq!(rc, expected_race_control)
	)
	
    }

    #[test]
    fn failed_request_race_controls() {
	let race_control: Option<Vec<RaceControl>> = API.get_race_control(Some(Category::Flag), Some(Flag::BlackAndWhite), Some(DriverNumber::new(150)), None, None);
	assert_eq!(race_control, None)
    }
    #[test]
    fn category_serialization() {
	let s = "\"Flag\"";
	let category = Category::Flag;

	let deserialized = serde_json::from_str::<Category>(&s).unwrap();
	println!("Deserialized: {:?}", deserialized); // Outputs: Flag
	assert_eq!(category, deserialized);
	
	let serialized = serde_json::to_string(&category).unwrap();
	println!("Serialized: {}", serialized); // Outputs: "Flag"
	assert_eq!(s, serialized);	
    }

    #[test]
    fn flag_serialization() {
	let s = "\"BLACK AND WHITE\"";
	let flag = Flag::BlackAndWhite;

	let deserialized = serde_json::from_str::<Flag>(&s).unwrap();
	println!("Deserialized: {:?}", deserialized); // Outputs: Flag
	assert_eq!(flag, deserialized);
	
	let serialized = serde_json::to_string(&flag).unwrap();
	println!("Serialized: {}", serialized); // Outputs: "Flag"
	assert_eq!(s, serialized);
    }
}
