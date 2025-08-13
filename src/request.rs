use crate::structs::body::Body;
use crate::structs::content_type::ContentType;
use crate::structs::header::Header;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Request {
    header: Header,
    body: Body,
}

impl Request {
    // 从流中构建
    pub fn from_stream(stream: impl std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf_reader = BufReader::new(stream);

        // 收集 header（直到第一个空行）
        let header = Self::header_build(&mut buf_reader);

        let header = Header::from(header);

        // 剩余部分作为 body
        let ct = header.try_get_type()?;

        let body = Self::body_build(&mut buf_reader, ct)?;

        Ok(Request {
            header: header,
            body: body,
        })
    }

    fn body_build(
        reader: &mut impl std::io::Read,
        ct: ContentType,
    ) -> Result<Body, Box<dyn std::error::Error>> {
        if ct == ContentType::JSON
            || ct == ContentType::FORM
            || ct == ContentType::XML
            || ct == ContentType::TEXT
        {
            let mut data = String::new();
            _ = reader.read_to_string(&mut data)?;
            Ok(Body::Text(data))

        } else {
            let mut data = Vec::new();
            _ = reader.read_to_end(&mut data)?;
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
}
