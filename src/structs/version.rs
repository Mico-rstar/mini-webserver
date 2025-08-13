/// 表示 HTTP 版本的枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    Http1_0,  // HTTP/1.0
    Http1_1,  // HTTP/1.1
    Http2,    // HTTP/2
    Http3,    // HTTP/3
}

impl HttpVersion {
    /// 从字符串解析 HTTP 版本（如 "HTTP/1.1" -> `HttpVersion::Http1_1`）
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "HTTP/1.0" => Some(HttpVersion::Http1_0),
            "HTTP/1.1" => Some(HttpVersion::Http1_1),
            "HTTP/2"   => Some(HttpVersion::Http2),
            "HTTP/3"   => Some(HttpVersion::Http3),
            _ => None,
        }
    }

    /// 转换为标准格式的字符串（如 `HttpVersion::Http1_1` -> "HTTP/1.1"）
    pub fn to_string(&self) -> String {
        match self {
            HttpVersion::Http1_0 => "HTTP/1.0".to_string(),
            HttpVersion::Http1_1 => "HTTP/1.1".to_string(),
            HttpVersion::Http2   => "HTTP/2".to_string(),
            HttpVersion::Http3   => "HTTP/3".to_string(),
        }
    }
}

/// 为 `HttpVersion` 实现 `Display` trait，方便打印
impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// 从 `&str` 自动解析为 `HttpVersion`（通过 `?` 操作符）
impl TryFrom<&str> for HttpVersion {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HttpVersion::from_str(value)
            .ok_or_else(|| format!("Invalid HTTP version: {}", value))
    }
}