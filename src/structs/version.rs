/// 表示 HTTP 版本的枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    Http1_0,  // HTTP/1.0
    Http1_1,  // HTTP/1.1
    Http2,    // HTTP/2
    Http3,    // HTTP/3
    Unknown,
}

impl HttpVersion {
    /// 从字符串解析 HTTP 版本（如 "HTTP/1.1" -> `HttpVersion::Http1_1`）
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "HTTP/1.0" => Self::Http1_0,
            "HTTP/1.1" => Self::Http1_1,
            "HTTP/2"   => Self::Http2,
            "HTTP/3"   => Self::Http3,
            _ => Self::Unknown,
        }
    }

    /// 转换为标准格式的字符串（如 `HttpVersion::Http1_1` -> "HTTP/1.1"）
    pub fn to_string(&self) -> String {
        match self {
            Self::Http1_0 => "HTTP/1.0".to_string(),
            Self::Http1_1 => "HTTP/1.1".to_string(),
            Self::Http2   => "HTTP/2".to_string(),
            Self::Http3   => "HTTP/3".to_string(),
            Self::Unknown => "Unknown version".to_string(),

        }
    }
}

/// 为 `HttpVersion` 实现 `Display` trait，方便打印
impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

