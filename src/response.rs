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
