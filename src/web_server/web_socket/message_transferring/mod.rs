use ws_request::UserWsRequest;

use crate::{core::core_state::CoreState, logger};

pub mod ws_request;

pub fn parse_request(request: String) -> Option<UserWsRequest> {
    let result = serde_json::from_str(&request);
    match result {
       Ok(obj)      => obj,
       Err(error)   => {
            logger::error(format!("Failed to parse reqeust={}, error occured={}", request, error).as_str());
            None
       }
   }
}

pub fn create_response(state: CoreState) -> Option<String> {
    let result = serde_json::to_string(&state);
    match result {
        Ok(str) => Some(format!("{{\"status\": {}}}", str)),
        Err(error) => {
            logger::error(format!("Faield to serialzie response state={:?}, error={}", state, error).as_str());
            None
        }
    }
}
