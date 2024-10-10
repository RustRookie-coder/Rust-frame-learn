use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResultTo<T> {
    pub status: i32,
    pub message: String,
    pub data: T,
}

/**
在Rust中，Sync和Send是两个关于所有权和生命周期的安全性属性。
Sync：表示一个类型的引用可以在多个线程之间共享。
Send：表示一个类型可以安全地发送到另一个线程。
where T: Sync + Send表示，对于泛型类型T，要求它既是Sync（可以在多线程之间共享），也是Send（可以安全地发送到另一个线程）。
**/
impl<T> ResultTo<T>
    where T: Sync + Send {
    pub fn success_response(data: T) -> ResultTo<T> {
        ResultTo {
            status: 200,
            message: "success".to_string(),
            data,
        }
    }
}
