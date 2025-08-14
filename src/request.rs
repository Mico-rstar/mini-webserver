use crate::structs::body::Body;
use crate::structs::content_type::ContentType;
use crate::structs::header::Header;
use crate::structs::request_line::RequestLine;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    request_line: RequestLine,
    header: Header,
    body: Body,
}

impl Request {
    // 从流中构建
    pub fn from_stream(
        stream: &mut impl std::io::Read,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf_reader = BufReader::new(stream);

        let rl = Self::request_line_build(&mut buf_reader)?;
        // 收集 header（直到第一个空行）
        let header_map = Self::header_build(&mut buf_reader);
        let content_length = header_map
            .get("Content-Length")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);
        let header = Header::from(header_map);

        // 剩余部分作为 body
        if let Ok(ct) = header.try_get_type() {
            let body = Self::body_build(&mut buf_reader, ct, content_length)?;
            Ok(Request {
                request_line: rl,
                header: header,
                body: body,
            })
        } else {
            // 缺少Content-Type字段
            // 认为不存在body
            Ok(Request {
                request_line: rl,
                header: header,
                body: Body::None,
            })
        }
    }

    fn request_line_build(reader: &mut impl BufRead) -> Result<RequestLine, Box<dyn std::error::Error>> {
        let mut buf = String::new();
        let _ = reader.read_line(&mut buf)?;
        let rl = RequestLine::from_str(&buf)?;
        Ok(rl)
    }

    fn body_build(
        reader: &mut impl std::io::Read,
        ct: ContentType,
        len: u64,
    ) -> Result<Body, Box<dyn std::error::Error>> {
        if ct == ContentType::JSON
            || ct == ContentType::FORM
            || ct == ContentType::XML
            || ct == ContentType::TEXT
        {
            let mut data = String::new();
            reader.take(len).read_to_string(&mut data)?;
            Ok(Body::Text(data))
        } else {
            let mut data = Vec::new();
            reader.take(len).read_to_end(&mut data)?;
            Ok(Body::Binary(data))
        }
    }

    fn header_build(reader: &mut impl BufRead) -> HashMap<String, String> {
        reader
            .lines()
            .map_while(Result::ok)
            .take_while(|line| !line.is_empty())
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                match (parts.next(), parts.next()) {
                    // 移除键的前后空白和值的前导空白
                    (Some(key), Some(value)) => {
                        Some((key.trim().to_string(), value.trim_start().to_string()))
                    }
                    _ => None, // 忽略无效行
                }
            })
            .collect()
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn request_line(&self) -> &RequestLine {
        &self.request_line
    }
}
