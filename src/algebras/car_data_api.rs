use std::vec::Vec;
use crate::algebras::http_requester::TelemetryHttpRequester;
use crate::algebras::http_requester::HttpRequester;
use crate::types::session::Session;


pub trait CarDataApi {
    //TODO: this should return a result or future with an error channel
    fn get_session(&self, country_name: &str, session_name: &str, year: u32) -> Option<Vec<Session>>;
}

pub struct CarDataApiImpl<'a> {
    pub http_requester: &'a TelemetryHttpRequester,
    pub uri: &'a str,
}

impl CarDataApi for CarDataApiImpl<'_> {
        fn get_session(&self, country_name: &str, session_name: &str, year: u32) -> Option<Vec<Session>> {
	let request_url = self.uri.to_owned() + &format!("/v1/sessions?country_name={country_name}&session_name={session_name}&year={year}");
	match self.http_requester.get::<Vec<Session>>(&request_url) {
	    Ok(sessions) if sessions.is_empty() => None,
	    Ok(sessions) => Some(sessions),
	    Err(_) => None,
	}
    }
	
}
