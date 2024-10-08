use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::{get, put};
use redis::AsyncCommands;
use serde_json::{from_str, json};
use validator::Validate;
use crate::app_state::AppState;

use crate::dto::dto::{FilterUserDto, NameUpdateDto, RequestQueryDto};
use crate::error::errors::HttpError;
use crate::middleware::auth_middleware::JWTAuthMiddleware;
use crate::repository::users::UserExt;
use crate::result::ResultTo;

pub fn users_handler() -> Router {
    Router::new()
        .route("/list", get(get_users))
        .route("/update", put(update_user_name))
}

async fn get_users(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let mut redis_client = app_state.redis.clone();

    let cache_data: String = redis_client.get("USER")
        .await
        .unwrap_or("".to_string());

    if cache_data.len() > 0 {
        let data = from_str::<Vec<FilterUserDto>>(&cache_data).unwrap();
        return Ok(Json(ResultTo::success_response(data)))
    }

    // if not find data in cache then request from database
    let users = app_state.db_client.get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let filter_user = FilterUserDto::convert_to_users(&users);

    let cache = json!(&filter_user).to_string();
    redis_client.set("USER", cache)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(ResultTo::success_response(filter_user)))
}

async fn update_user_name(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json<NameUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = &user.user_id;
    let user = app_state.db_client.update_user_name(user_id.to_string(), &body.name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut redis_client = app_state.redis.clone();
    redis_client.del("USER")
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(ResultTo::success_response(FilterUserDto::convert_to_dto_user(&user))))
}
