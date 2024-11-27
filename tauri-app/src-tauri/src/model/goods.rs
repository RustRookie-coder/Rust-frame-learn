use chrono::{DateTime, Utc};

/// Rust 提供了 原始标识符 功能，允许你使用关键字作为变量或字段名。只需要在关键字前加上 r# 即可。
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Skus {
    id: i32,
    name: String,
    r#type: i32,
    status: i32,
    order: i32,
    default: String,
    create_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
}

impl Skus {
    #[inline]
    pub fn new(id: i32,
                      name: String,
                      r#type: i32,
                      status: i32,
                      order: i32,
                      default: String,
                      create_time: DateTime<Utc>,
                      update_time: DateTime<Utc>) -> Self {
        Skus {
            id,
            name,
            r#type,
            status,
            order,
            default,
            create_time,
            update_time,
        }
    }
}