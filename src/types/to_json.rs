pub trait ToJson {
    fn to_json(&self) -> Option<String>;
}
