use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "id")]
    pub iso:     String,
    pub city:    String,
    pub country: String,
    pub ping_ms: i32,
}
