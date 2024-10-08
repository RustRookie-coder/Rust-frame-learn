use std::fmt;
use std::fmt::{Display, Formatter};

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    BadRequest,
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidToken,
    InvalidHashFormat,
    ServerError,
    MissingCredentials,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str().to_owned())
    }
}

// impl fmt::Display for ErrorMessage {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", serde_json::to_string(&self).unwrap())
//     }
// }

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::BadRequest => "request params not correct".to_owned(),
            ErrorMessage::EmptyPassword => "Password is required".to_owned(),
            ErrorMessage::ExceededMaxPasswordLength(length) => format!("Password must be at most {} character long", length),
            ErrorMessage::HashingError => "Error occurred while hashing password".to_owned(),
            ErrorMessage::InvalidToken => "Invalid token".to_owned(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::ServerError => "Internal server error".to_owned(),
            ErrorMessage::MissingCredentials => "Missing credentials".to_owned(),
            ErrorMessage::WrongCredentials => "Wrong credentials".to_owned(),
            ErrorMessage::EmailExist => "Email already exists".to_owned(),
            ErrorMessage::UserNoLongerExist => "User no longer exists".to_owned(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_owned(),
            ErrorMessage::PermissionDenied => "Permission denied".to_owned(),
            ErrorMessage::UserNotAuthenticated => "User not authenticated".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: self.status.to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }
    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
