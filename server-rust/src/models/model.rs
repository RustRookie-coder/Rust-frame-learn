use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// the output to our `create_user` handlers
#[derive(Debug, Deserialize, Serialize, FromRow, Type, Clone)]
pub struct LivingLabUser {
    pub id: i32,
    pub user_id: String,
    pub openid: String,
    pub session_key: String,
    pub name: String,
    pub email: String,
    pub password: String,
    // pub role: String,
    pub verification_token: Option<String>,
    // pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "living_lab_users_role", rename_all = "lowercase")]
pub enum LivingLabUsersRole {
    Admin,
    User
}

impl LivingLabUsersRole {
    pub fn to_str(&self) -> &str {
        match self {
            LivingLabUsersRole::Admin => "admin",
            LivingLabUsersRole::User => "user",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, Type, Clone)]
pub struct LivingLabAccessLogs {
    pub id: i32,
    pub name: String,
    pub user_id: String,
    pub username: String,
    pub ip_addr: i64,
    pub method: String,
    pub request_url: String,
    pub device: String,
    pub execution_time: i64,
    pub error: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}


pub struct Images {
    data: Vec<u8>,
}

impl Images {
    fn process(&self) {
        let image = image::load_from_memory(&self.data).unwrap();
        //进行图片处理，例如调整大小等
    }
}


/**
在Rust中，布尔类型（bool）有两种取值：true和false。如果将bool类型转换为整数，true将被赋值为1，而false将被赋值为0
 */
#[derive(Debug, Deserialize)]
pub struct AddCounter {
    pub name: String,
    pub value: i32,
    pub step: i32,
    pub input_step: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCounter {
    pub name: String,
    pub step: i32,
    pub input_step: bool,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Counter {
    pub id: i32,
    pub user_id: String,
    pub name: String,
    pub value: i32,
    pub step: i32,
    pub input_step: i32,
    pub sequence: i32,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct CounterRecord {
    pub id: i32,
    pub counter_id: i32,
    pub step: i32,
    pub begin: i32,
    pub end: i32,
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Questionnaire {
    pub id: i32,
    pub user_id: String,
    pub url: String,
}
