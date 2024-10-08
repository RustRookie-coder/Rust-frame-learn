use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::Next
    ,
};
use axum::Extension;
use axum::response::IntoResponse;
use reqwest::header;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::app_state::AppState;

use crate::dto::dto::{CreateLog, FilterUserDto};
use crate::error::errors::{ErrorMessage, HttpError};
use crate::models::model::LivingLabUser;
use crate::repository::log::LogExt;
use crate::repository::users::UserExt;
use crate::utils::token;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: FilterUserDto,
}

pub async fn auth(
    Extension(app_state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next, ) -> Result<impl IntoResponse, HttpError> {
    let token = req.headers().get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
        if auth_value.starts_with("Bearer ") {
            Some(auth_value[7..].to_owned())
        } else { None }
    });

    let token = token.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::TokenNotProvided.to_string())
    })?;

    let token_sub = match token::decode_token(token, app_state.env.jwt_secret.as_bytes()) {
        Ok(token_details) => token_details,
        Err(_) => {
            return Err(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()));
        }
    };

    let user_id = token_sub.parse::<String>().unwrap();

    let user = app_state.db_client
        .get_user_by_user_id(Option::from(user_id.as_str()))
        .await.map_err(|_| {
        HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
    })?;

    info!("user_id: {:?} log in success ******************", user_id);
    req.extensions_mut().insert(JWTAuthMiddleware{
        user: FilterUserDto::convert_to_dto_user(&user),
    });

    let user_log = user.clone();

    let log = CreateLog::new(String::from(""), user_id.clone(),
                             user_log.name, 0, req.method().to_string(),
                             req.uri().to_string(), String::from(""),
                             0, String::from(""));

    app_state.db_client.create_access_log(log)
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::BadRequest.to_string()))?;

    Ok(next.run(req).await)
}
