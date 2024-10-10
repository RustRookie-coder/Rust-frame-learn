use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{async_trait, Json, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,     //过期时间戳
}

#[warn(dead_code)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

#[warn(dead_code)]
pub const KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[warn(dead_code)]
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[warn(dead_code)]
impl Claims {
    pub fn new(sub: String) -> Self {
        let exp: SystemTime = SystemTime::now() + Duration::from_secs(15 * 24 * 60 * 60); //after 15 days expire
        let exp: usize = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Claims { sub, exp }
    }
}

#[warn(dead_code)]
pub struct Uid(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Uid
    where S: Send + Sync {
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts.extract::<TypedHeader<Authorization<Bearer>>>().await.map_err(|_| AuthError::InvalidToken)?;
        //Decode the user data
        let token_data = decode::<Claims>(bearer.token(),
                                          // &DecodingKey::from_secret(b"secret"),
                                          &KEYS.decoding,
                                          &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        let user_id = token_data.claims.sub.parse::<String>().unwrap();
        Ok(Uid(user_id))
    }
}

#[warn(dead_code)]
#[derive(Debug)]
pub enum AuthError {
    TokenCreation,
    WrongCredentials,
    MissingCredentials,
    InvalidToken,
}

#[warn(dead_code)]
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
