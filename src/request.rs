use crate::structs::body::Body;
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
    pub fn from_stream(stream: impl std::io::Read) -> Self {
        let buf_reader = BufReader::new(stream);

        let mut lines = buf_reader.lines();

        // 收集 header（直到第一个空行）
        let header: HashMap<String, String> = lines
            .by_ref()
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
            .collect();

        // 剩余部分作为 body
        let body = lines.map_while(Result::ok).collect::<Vec<_>>().join("\n");

        Request {
            header: Header::from(header),
            body: Body::Text(body),
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

}
