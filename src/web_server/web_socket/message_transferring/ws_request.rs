use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWsRequest {
    pub request_type: RequestType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestType {
    Connect,
    Disconnect,
    Restart,
}
