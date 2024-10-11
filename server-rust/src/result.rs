use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResultTo {
    pub status: i32,
    pub message: String,
    pub data: Value,
}

/**
在Rust中，Sync和Send是两个关于所有权和生命周期的安全性属性。
Sync：表示一个类型的引用可以在多个线程之间共享。
Send：表示一个类型可以安全地发送到另一个线程。
where T: Sync + Send表示，对于泛型类型T，要求它既是Sync（可以在多线程之间共享），也是Send（可以安全地发送到另一个线程）。
**/
impl ResultTo
    where Value: Sync + Send {
    pub fn success_response(data: Value) -> ResultTo {
        ResultTo {
            status: 200,
            message: "success".to_string(),
            data,
        }
    }
}
