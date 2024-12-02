use chrono::{DateTime, Utc};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Coupons {
    id: i32,
    name: String,
    r#type: i8,
    value: String,
    total: i32,
    used: i32,
    min_price: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    status: i8,
    create_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
    order: i32,
    desc: String,
}

impl Coupons {
    #[inline]
    pub fn new(id: i32,
               name: String,
               r#type: i8,
               value: String,
               total: i32,
               used: i32,
               min_price: String,
               start_time: DateTime<Utc>,
               end_time: DateTime<Utc>,
               status: i8,
               create_time: DateTime<Utc>,
               update_time: DateTime<Utc>,
               order: i32,
               desc: String) -> Self {
        Coupons {
            id,
            name,
            r#type,
            value,
            total,
            used,
            min_price,
            start_time,
            end_time,
            status,
            create_time,
            update_time,
            order,
            desc,
        }
    }
}