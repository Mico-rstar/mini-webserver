#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Ok,                // 200
    BadRequest,        // 400
    NotFound,          // 404
    InternalServerError, // 500
    Forbidden,         // 403
}

impl Status {
    /// 获取状态码的数值（如 200）
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::Forbidden => 403,
        }
    }

    /// 获取状态描述（如 "OK"）
    pub fn description(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
            Self::Forbidden => "Forbidden",
        }
    }

    /// 从数值构造状态码（返回 Option）
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            200 => Some(Self::Ok),
            400 => Some(Self::BadRequest),
            404 => Some(Self::NotFound),
            500 => Some(Self::InternalServerError),
            403 => Some(Self::Forbidden),
            _ => None,
        }
    }
}