use chrono::{DateTime, Utc};
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::model::LivingLabUser;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(length(min = 6, message = "code is required"))]
    pub code: String,
    pub login_source: LoginPlatform,
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoginPlatform {
    #[default]
    WeChat,
    Github,
    Apple,
    Agora,
    Google,
    Phone,
    Email,
}

#[derive(Deserialize)]
pub struct WxUser {
    pub openid: String,
    pub session_key: String,
}

impl Default for WxUser {
    fn default() -> Self {
        WxUser {
            // openid: String::from(uuid::Uuid::new_v4()),
            // session_key: String::from(uuid::Uuid::new_v4()),
            openid: String::from("738b2404-920a-457d-a6a2-1457845a625c"),
            session_key: String::from("40a14833-60be-4d59-a160-29db008d4fd8"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterUserDto {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub name: String,
    pub email: String,
    // pub role: String,
    pub verification_token: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn convert_to_dto_user(user: &LivingLabUser) -> Self {
        FilterUserDto {
            user_id: user.user_id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            verification_token: user.verification_token.clone(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn convert_to_users(user: &[LivingLabUser]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::convert_to_dto_user).collect()
    }
}

impl ToRedisArgs for FilterUserDto {
    fn write_redis_args<W>(&self, out: &mut W)
        where W: ?Sized + RedisWrite
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize Planet as string"))
    }
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateLog {
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    pub ip_addr: i64,
    pub method: String,
    pub request_url: String,
    pub device: String,
    pub execution_time: i64,
    pub error: String,
}

impl CreateLog {
    pub fn new(name: String, user_id: String, username: String,
               ip_addr: i64, method: String, request_url: String,
               device: String, execution_time: i64, error: String) -> Self {
        CreateLog {
            name,
            user_id,
            username,
            ip_addr,
            method,
            request_url,
            device,
            execution_time,
            error,
        }
    }
}
