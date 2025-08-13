use crate::structs::version::HttpVersion;
use crate::structs::status::Status;

#[derive(Debug, Clone)]
pub struct StatusLine {
    version: HttpVersion,
    status: Status,
}

impl StatusLine {
    pub fn new(_version: HttpVersion, _status: Status) -> Self
    {
        StatusLine
        {
            version: _version,
            status: _status,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.version.to_string(), self.status.code(), self.status.description())
    }
}


impl std::fmt::Display for StatusLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

