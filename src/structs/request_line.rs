use std::str::FromStr;
use thiserror::Error;

use crate::structs::version::HttpVersion;
use crate::structs::method::{Method, MethodError};



#[derive(Debug, PartialEq)]
pub struct RequestLine {
    pub method: Method,
    pub uri: String,
    pub version: HttpVersion,
}

#[derive(Error, Debug)]
pub enum RequestLineError {
    #[error("Invalid request line format: not enough parts")]
    InvalidFormat,
    #[error("Invalid HTTP method: {0}")]
    InvalidMethod(#[from] MethodError),
}

impl FromStr for RequestLine {
    type Err = RequestLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let method_str = parts.next().ok_or(RequestLineError::InvalidFormat)?;
        let uri_str = parts.next().ok_or(RequestLineError::InvalidFormat)?;
        let version_str = parts.next().ok_or(RequestLineError::InvalidFormat)?;

        if parts.next().is_some() {
            return Err(RequestLineError::InvalidFormat);
        }

        let method = Method::from_str(method_str)?;
        let uri = uri_str.to_string();
        let version = HttpVersion::from_str(version_str);

        Ok(RequestLine {
            method,
            uri,
            version,
        })
    }
}

impl std::fmt::Display for RequestLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.method, self.uri, self.version)
    }
}