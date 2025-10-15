use hyper::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum LunoxError {
    Http(hyper::Error),
    Json(serde_json::Error),
    UrlDecode(serde_urlencoded::de::Error),
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    BadRequest(String),
    NotFound(String),
    Internal(String),
    Unauthorized(String),
    Forbidden(String),
    Custom(StatusCode, String),
}

impl LunoxError {
    /// Get the HTTP status code for error
    pub fn status_code(&self) -> StatusCode {
        match self {
            LunoxError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LunoxError::Json(_) => StatusCode::BAD_REQUEST,
            LunoxError::UrlDecode(_) => StatusCode::BAD_REQUEST,
            LunoxError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LunoxError::Utf8(_) => StatusCode::BAD_REQUEST,
            LunoxError::BadRequest(_) => StatusCode::BAD_REQUEST,
            LunoxError::NotFound(_) => StatusCode::NOT_FOUND,
            LunoxError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LunoxError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            LunoxError::Forbidden(_) => StatusCode::FORBIDDEN,
            LunoxError::Custom(code, _) => *code,
        }
    }

    /// Get the message of Errors
    pub fn message(&self) -> String {
        match self {
            LunoxError::Http(e) => format!("HTTP Error: {}", e),
            LunoxError::Json(e) => format!("JSON Error: {}", e),
            LunoxError::UrlDecode(e) => format!("Cannot decode URL because: {}", e),
            LunoxError::Io(e) => format!("IO Error: {}", e),
            LunoxError::Utf8(e) => format!("UTF-8 Error: {}", e),
            LunoxError::BadRequest(msg) => msg.clone(),
            LunoxError::NotFound(msg) => msg.clone(),
            LunoxError::Internal(msg) => msg.clone(),
            LunoxError::Unauthorized(msg) => msg.clone(),
            LunoxError::Forbidden(msg) => msg.clone(),
            LunoxError::Custom(_, msg) => msg.clone(),
        }
    }
} // impl LunoxError

impl fmt::Display for LunoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
} // impl fmt::Display

impl std::error::Error for LunoxError {}

impl From<serde_json::Error> for LunoxError {
    fn from(err: serde_json::Error) -> Self {
        LunoxError::Json(err)
    }
} // impl From<serde_json::Error>

impl From<serde_urlencoded::de::Error> for LunoxError {
    fn from(err: serde_urlencoded::de::Error) -> Self {
        LunoxError::UrlDecode(err)
    }
} // impl From<serde_urlencoded::de::Error>

impl From<std::io::Error> for LunoxError {
    fn from(err: std::io::Error) -> Self {
        LunoxError::Io(err)
    }
} // impl From<std::io::Error>

impl From<std::str::Utf8Error> for LunoxError {
    fn from(err: std::str::Utf8Error) -> Self {
        LunoxError::Utf8(err)
    }
} // impl From<std::str::Utf8Error>

impl From<http::Error> for LunoxError {
    fn from(err: http::Error) -> Self {
        LunoxError::Internal(format!("HTTP Error: {}", err))
    }
} // impl From<http::Error>
