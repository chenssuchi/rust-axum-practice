use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    LoginFailed,

    // Auth errors
    AuthFailedNoAuthTokenCookie,
    AuthFailedTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    // Model errors
    TicketDeleteFailIdNotFound { id: u64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        match self {
            Error::LoginFailed => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
            Error::AuthFailedNoAuthTokenCookie => {
                (StatusCode::UNAUTHORIZED, "Auth token cookie not found").into_response()
            }
            Error::AuthFailedTokenWrongFormat => {
                (StatusCode::UNAUTHORIZED, "Auth token format not supported").into_response()
            }
            Error::AuthFailCtxNotInRequestExt => (
                StatusCode::UNAUTHORIZED,
                "Auth token context not in request ext",
            )
                .into_response(),
            Error::TicketDeleteFailIdNotFound { id } => {
                (StatusCode::NOT_FOUND, format!("Ticket id {} not found", id)).into_response()
            }
        }
    }
}
