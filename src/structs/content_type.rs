#[derive(Debug, thiserror::Error)]
pub enum ContentTypeError {
    #[error("Unsupported content type: {0}")]
    UnsupportedType(String),
    
    #[error("Missing boundary in form-data")]
    MissingBoundary,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ContentType {
    FORM,     // application/x-www-form-urlencoded
    FORMDATA(String), // multipart/form-data
    JSON,     // application/json
    XML,      // text/xml
    HTML,     // text/html
    TEXT,     // text/plain
    STREAM,   //application/octet-stream
    CSS,
    JS,
    PNG,
    JPEG,
    GIF,
    ICO,
}

impl ContentType {
    pub fn to_string(&self) -> String {
        match self {
            Self::FORM => String::from("application/x-www-form-urlencoded"),
            Self::FORMDATA(boundary) => format!("multipart/form-data; boundary={}", boundary),
            Self::JSON => String::from("application/json"),
            Self::XML => String::from("text/xml"),
            Self::TEXT => String::from("text/plain"),
            Self::STREAM => String::from("application/octet-stream"),
            Self::HTML => String::from("text/html"),
            Self::CSS => String::from("text/css"),
            Self::JS => String::from("application/javascript"),
            Self::PNG => String::from("image/png"),
            Self::JPEG => String::from("image/jpeg"),
            Self::GIF => String::from("image/gif"),
            Self::ICO => String::from("image/x-icon"),
        }
    }

     pub fn try_from(ctype: &str) -> Result<Self, ContentTypeError> {
        let normalized = ctype.trim();
        
        match normalized.to_ascii_lowercase().as_str() {
            "application/x-www-form-urlencoded" => Ok(Self::FORM),
            "application/json" => Ok(Self::JSON),
            "text/xml" => Ok(Self::XML),
            "text/plain" => Ok(Self::TEXT),
            "application/octet-stream" => Ok(Self::STREAM),
            "text/html" => Ok(Self::HTML),
            "text/css" => Ok(Self::CSS),
            "application/javascript" => Ok(Self::JS),
            "image/png" => Ok(Self::PNG),
            "image/jpeg" => Ok(Self::JPEG),
            "image/gif" => Ok(Self::GIF),
            "image/x-icon" => Ok(Self::ICO),
            s if s.starts_with("multipart/form-data") => {
                Self::parse_form_data(s).ok_or(ContentTypeError::MissingBoundary)
            },
            _ => Err(ContentTypeError::UnsupportedType(normalized.to_string())),
        }
    }

    fn parse_form_data(s: &str) -> Option<Self> {
        let mut parts = s.split(';').map(str::trim);
        
        // 验证主类型
        if !matches!(parts.next(), Some("multipart/form-data")) {
            return None;
        }

        // 提取 boundary
        parts.find_map(|part| {
            part.strip_prefix("boundary=")
                .map(|boundary| Self::FORMDATA(boundary.trim_matches(|c| c == '"' || c == '\'' ).to_string()))
        })
    }
}
