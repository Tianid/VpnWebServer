use std::str::FromStr;

#[derive(Debug)]
pub enum HttpMethod {
    GET, POST, DELETE, UPDATE, HEAD, PUT, OPTIONS, CONNECT, TRACE, PATCH
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET"       => Ok(HttpMethod::GET),
            "POST"      => Ok(HttpMethod::POST),
            "DELETE"    => Ok(HttpMethod::DELETE),
            "UPDATE"    => Ok(HttpMethod::UPDATE),
            "HEAD"      => Ok(HttpMethod::HEAD),
            "PUT"       => Ok(HttpMethod::PUT),
            "OPTIONS"   => Ok(HttpMethod::OPTIONS),
            "CONNECT"   => Ok(HttpMethod::CONNECT),
            "TRACE"     => Ok(HttpMethod::TRACE),
            "PATCH"     => Ok(HttpMethod::PATCH),
            _           => Err(())
        }
    }
}
