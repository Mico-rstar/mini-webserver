use crate::structs::body::Body;
use crate::structs::content_type::ContentType;
use crate::structs::header::Header;
use crate::structs::status::Status;
use crate::structs::status_line::StatusLine;
use crate::structs::version::HttpVersion;

#[derive(Debug, Clone)]
pub struct Response {
    status_line: StatusLine,
    header: Header,
    body: Body,
}

impl Response {
    // 提供默认的响应头构建
    pub fn header_build(host: String, ctype: ContentType) -> Header {
        let mut h = Header::new();
        h.set(String::from("Content-Type"), ctype.to_string())
            .set(String::from("Server"), host)
            .set("Content-Length".to_string(), "0".to_string());
        h
    }

    pub fn new(host: &str, status: Status, ctype: ContentType) -> Self {
        Response {
            status_line: StatusLine::new(HttpVersion::Http1_1, status),
            header: Self::header_build(String::from(host), ctype),
            body: Body::None,
        }
    }

    // 设置响应体
    pub fn set_body(&mut self, _body: Body) {
        self.body = _body;
        // 设置请求头Content-Length字段
        self.header.set("Content-Length".to_string(), self.body.len().to_string());
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut response_bytes = Vec::new();

        // Status line
        response_bytes.extend_from_slice(self.status_line.to_string().as_bytes());
        response_bytes.extend_from_slice(b"\r\n");

        // Headers
        response_bytes.extend_from_slice(self.header.to_string().as_bytes());
        response_bytes.extend_from_slice(b"\r\n");

        // Separator
        response_bytes.extend_from_slice(b"\r\n");

        // Body
        match &self.body {
            Body::Text(text) => response_bytes.extend_from_slice(text.as_bytes()),
            Body::Binary(binary) => response_bytes.extend_from_slice(binary),
            Body::None => (),
        }

        response_bytes
    }
}
