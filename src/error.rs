use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "unhandled_client_error").into_response()
    }
}
