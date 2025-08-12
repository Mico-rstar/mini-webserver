pub enum Contype {
    FORM,     // application/x-www-form-urlencoded
    FORMDATA(String), // multipart/form-data
    JSON,     // application/json
    XML,      // text/xml
    TEXT,     // text/plain
    STREAM,   //application/octet-stream
}

impl Contype {
    pub fn to_string(&self) -> String {
        match self {
            Contype::FORM => String::from("application/x-www-form-urlencoded"),
            Contype::FORMDATA(boundary) => format!("multipart/form-data; boundary={}", boundary),
            Contype::JSON => String::from("application/json"),
            Contype::XML => String::from("text/xml"),
            Contype::TEXT => String::from("text/plain"),
            Contype::STREAM => String::from("application/octet-stream"),
        }
    }
}