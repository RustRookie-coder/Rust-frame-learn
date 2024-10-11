use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::{Extension, Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;
use crate::app_state::AppState;

use crate::dto::dto::{LoginUserDto, WxUser};
use crate::error::errors::{ErrorMessage, HttpError};
use crate::repository::users::UserExt;
use crate::routes::jwt::{AuthError, Claims, KEYS};
use crate::utils::token::create_token;

pub fn auth_handler() -> Router {
    Router::new()
        .route("/login", post(login))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    responses(
        (status=200,body=AuthBody, description="token created"),
        (status=404, description="not found")
    )
)]
async fn login(Extension(app_state): Extension<Arc<AppState>>,
                   Json(body): Json<LoginUserDto>) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let wx_user: WxUser = wx_login(body.code).await?;
    let user = app_state.db_client
        .get_user_by_openid(Option::from(wx_user.openid.as_str()),
                            Option::from(wx_user.session_key.as_str()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    // let claims = Claims::new(user.user_id.to_string());
    // let token = encode(&Header::default(),
    //                    &claims,
    //                    // &EncodingKey::from_secret(b"secret"))
    //                    &KEYS.encoding,)
    //     .map_err(|_| AuthError::TokenCreation)?;
    let token = create_token(&user.user_id,
                 &app_state.env.jwt_secret.as_bytes(),
                 15 * 24 * 60 * 60).map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;
    Ok(Json(AuthBody::new(String::from(token))))
}

async fn wx_login(code: String) -> Result<WxUser, HttpError> {
    /**
        todo ==> need verify code is authority or not
    */
    // let app_id = env::var("APP_ID").unwrap();
    // let app_secret = env::var("APP_SECRET").unwrap();
    // let endpoint = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", app_id, app_secret, code);
    // let resp = reqwest::get(endpoint).await
    //     .map_err(|_| AuthError::WrongCredentials)?
    //     .json::<Value>().await.map_err(|_| AuthError::WrongCredentials)?;
    // info!("wx_login code: {}, resp: {}", code, resp);
    // let wx_user = serde_json::from_value::<WxUser>(resp).map_err(|_| AuthError::WrongCredentials)?;
    // if wx_user.openid.is_empty() {
    //     Err(AuthError::WrongCredentials)
    // } else { Ok(wx_user) }.expect("request wechat info success");
    //todo when app info ready use up code replace default object
    Ok(WxUser::default())
}

#[derive(Serialize, ToSchema)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
