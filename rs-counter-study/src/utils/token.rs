use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "iss", default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    pub sub: String,
    pub iat: usize,
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub audiences: Option<String>,
    exp: usize,     //过期时间戳
}

pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expires_in_second: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into())
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_second)).timestamp() as usize;

    let claims = Claims {
        issuer: Option::from(String::from("jerry.com")),
        sub: user_id.to_string(),
        iat,
        audiences: None,
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn decode_token(token: String, secret: &[u8]) -> Result<String, HttpError>{
    let decode = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default()
    ).map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(decode.claims.sub)
}
