pub struct WebServerConfiguration {
    pub address: String,
    pub port: String,
}

impl WebServerConfiguration {

    pub fn new(address: &str, port: &str) -> Self {
        Self { address: address.to_string(), port: port.to_string() }
    }
}
