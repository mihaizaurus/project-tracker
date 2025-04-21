use axum::{
    http::StatusCode, 
    response::{IntoResponse, Response},
    Json
};
use serde::Serialize;
use serde_json::json;

use project_tracker_core::id::ParseIdError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error { // To be improved later
    LoginFail,
    ProjectError(String),
    ParseError(ParseIdError),
    InvalidPayload(String),
    DatabaseError,
    Multiple(Vec<Error>)
    // etc.
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<ParseIdError> for Error {
    fn from(err: ParseIdError) -> Self {
        Error::ParseError(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login Failed".into()),
            Error::ProjectError(error_string) => (StatusCode::BAD_REQUEST, error_string),
            Error::ParseError(_) => (StatusCode::BAD_REQUEST, "Parsing Error".into()),
            Error::InvalidPayload(error_string) => (StatusCode::UNAUTHORIZED, error_string),
            Error::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error".into()),
            Error::Multiple(_) => (StatusCode::BAD_REQUEST, "Multiple validation Errors".into()),
            // fallback
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled Error".into()) // Kept for validation for when more error types are added;
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}