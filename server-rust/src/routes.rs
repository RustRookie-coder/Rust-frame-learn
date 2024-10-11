use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use axum::{Extension, middleware, Router, routing::get};
use axum::http::{HeaderName, Request};
use axum::response::IntoResponse;
use axum::routing::{delete, post, put, Route};
use sqlx::PgPool;
use tower::{Service, ServiceBuilder};
use tower_http::{
    ServiceBuilderExt,
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse, DefaultOnRequest},
};
use tower_http::request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer};
use tracing::{info_span, Level};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use rs_counter_study::handlers::auth::auth_handler;
use rs_counter_study::handlers::files::files_handler;
use rs_counter_study::handlers::health::heath_handler;
use rs_counter_study::handlers::swagger::ApiDoc;
use rs_counter_study::handlers::users::users_handler;
use rs_counter_study::middleware::auth_middleware::auth;
use crate::AppState;
// use crate::routes::counter::{add, destroy, list, show, top, update};
// use crate::routes::counter_record::{add_counter_record, list_counter_record};

pub(crate) mod jwt;
mod counter;
mod counter_record;

pub fn create_router(app_state: Arc<AppState>)  -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");
    let api_route = Router::new().nest("/auth", auth_handler())
        .nest("/users", users_handler()
            .layer(middleware::from_fn(auth)))
        .nest("/", heath_handler())
        .nest("/files", files_handler())
        .layer(Extension(app_state))
        .layer(ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(
                x_request_id.clone(),
                ToMakeRequestId::default(),
            ))
            .layer(TraceLayer::new_for_http()
                // .make_span_with(DefaultMakeSpan::new().level(Level::INFO).include_headers(true),)
                .make_span_with(|request: &Request<_>| {
                    let req_id = request
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str()
                            .unwrap_or(""))
                        .unwrap_or("");
                    info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        reqid = ?req_id,
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)))
            .layer(PropagateRequestIdLayer::new(x_request_id))
        );
    Router::new().nest("/api/v1", api_route)
        .route("/", get(root))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
}

async fn root() -> &'static str {
    "Welcome the rust axum project: Hello, World!"
}

#[derive(Clone, Default)]
struct ToMakeRequestId {
    counter: Arc<AtomicU64>,
}

impl MakeRequestId for ToMakeRequestId {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_id = self.counter.fetch_add(1, Ordering::SeqCst)
            .to_string()
            .parse()
            .unwrap();
        Some(RequestId::new(request_id))
    }
}
