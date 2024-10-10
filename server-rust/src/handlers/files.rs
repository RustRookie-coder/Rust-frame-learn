use axum::{
    extract::Multipart,
    routing::post,
    Router,
};

async fn do_upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}
pub fn files_handler() -> Router {
    Router::new().route("/upload", post(do_upload))
}

