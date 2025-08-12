pub enum ContentType {
    FORM,     // application/x-www-form-urlencoded
    FORMDATA(String), // multipart/form-data
    JSON,     // application/json
    XML,      // text/xml
    TEXT,     // text/plain
    STREAM,   //application/octet-stream
}

impl ContentType {
    pub fn to_string(&self) -> String {
        match self {
            ContentType::FORM => String::from("application/x-www-form-urlencoded"),
            ContentType::FORMDATA(boundary) => format!("multipart/form-data; boundary={}", boundary),
            ContentType::JSON => String::from("application/json"),
            ContentType::XML => String::from("text/xml"),
            ContentType::TEXT => String::from("text/plain"),
            ContentType::STREAM => String::from("application/octet-stream"),
        }
    }
}