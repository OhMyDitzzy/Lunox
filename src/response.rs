use std::convert::Infallible;

use crate::errors::LunoxError;
use hyper::{Body, Response as HyperResponse, StatusCode};

pub type Response = HyperResponse<Body>;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        HyperResponse::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(self))
            .unwrap()
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        HyperResponse::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(self))
            .unwrap()
    }
}

impl IntoResponse for (StatusCode, String) {
    fn into_response(self) -> Response {
        HyperResponse::builder()
            .status(self.0)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(self.1))
            .unwrap()
    }
}

impl IntoResponse for (StatusCode, &'static str) {
    fn into_response(self) -> Response {
        HyperResponse::builder()
            .status(self.0)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Body::from(self.1))
            .unwrap()
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        HyperResponse::builder()
            .status(self)
            .body(Body::empty())
            .unwrap()
    }
}

impl IntoResponse for LunoxError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = serde_json::json!({
            "error": self.message(),
            "status": status.as_u16()
        });

        HyperResponse::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap()
    }
}

impl<T: IntoResponse> IntoResponse for Result<T, LunoxError> {
    fn into_response(self) -> Response {
        match self {
            Ok(resp) => resp.into_response(),
            Err(err) => err.into_response(),
        }
    }
}

impl IntoResponse for Infallible {
    fn into_response(self) -> Response {
        match self {}
    }
}

pub struct ResponseBuilder {
    response: Result<HyperResponse<Body>, http::Error>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            response: HyperResponse::builder().body(Body::empty()),
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.response = HyperResponse::builder().status(status).body(Body::empty());
        self
    }

    pub fn header<V>(mut self, key: &str, value: V) -> Self {}
}
