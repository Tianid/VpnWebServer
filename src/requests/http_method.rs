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





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_valid_methods_parse() {
        assert!(matches!("GET".parse::<HttpMethod>(),     Ok(HttpMethod::GET)));
        assert!(matches!("POST".parse::<HttpMethod>(),    Ok(HttpMethod::POST)));
        assert!(matches!("DELETE".parse::<HttpMethod>(),  Ok(HttpMethod::DELETE)));
        assert!(matches!("UPDATE".parse::<HttpMethod>(),  Ok(HttpMethod::UPDATE)));
        assert!(matches!("HEAD".parse::<HttpMethod>(),    Ok(HttpMethod::HEAD)));
        assert!(matches!("PUT".parse::<HttpMethod>(),     Ok(HttpMethod::PUT)));
        assert!(matches!("OPTIONS".parse::<HttpMethod>(), Ok(HttpMethod::OPTIONS)));
        assert!(matches!("CONNECT".parse::<HttpMethod>(), Ok(HttpMethod::CONNECT)));
        assert!(matches!("TRACE".parse::<HttpMethod>(),   Ok(HttpMethod::TRACE)));
        assert!(matches!("PATCH".parse::<HttpMethod>(),   Ok(HttpMethod::PATCH)));
    }

    #[test]
    fn invalid_method_returns_err() {
        assert!("get".parse::<HttpMethod>().is_err());
        assert!("Get".parse::<HttpMethod>().is_err());
        assert!("".parse::<HttpMethod>().is_err());
        assert!("INVALID".parse::<HttpMethod>().is_err());
    }

    #[test]
    fn method_is_case_sensitive() {
        assert!("get".parse::<HttpMethod>().is_err());
        assert!("post".parse::<HttpMethod>().is_err());
        assert!("Delete".parse::<HttpMethod>().is_err());
    }
}
