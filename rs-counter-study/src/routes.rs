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
use rs_counter_study::handlers::auth::auth_handler;
use rs_counter_study::handlers::health::heath_handler;
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
    // let api_route = Router::new()
    //     .route("/", get(root))
    //     .route("/health/checker", get(health_checker_handler))
    //     // .route("/login", post(login))
    //     .route("/counters", get(list).post(add))
    //     .route("/counters/:id", get(show).put(update).delete(destroy))
    //     .route("/counters/:id/top", post(top))
    //     .route("/counter_record/:counter_id", get(list_counter_record))
    //     .route("/counter_record", post(add_counter_record))
    //     // .routes("/api/notes/", post(create_note_handler))
    //     // .routes("/api/notes", get(note_list_handler))
    //     // .routes(
    //     //     "/api/notes/:id",
    //     //     get(get_note_handler)
    //     //         .patch(edit_note_handler)
    //     //         .delete(delete_note_handler),
    //     // )
    //     .with_state(pool)
    //     .layer(ServiceBuilder::new()
    //         .layer(SetRequestIdLayer::new(
    //             x_request_id.clone(),
    //             ToMakeRequestId::default(),
    //         ))
    //         .layer(TraceLayer::new_for_http()
    //             // .make_span_with(DefaultMakeSpan::new().level(Level::INFO).include_headers(true),)
    //             .make_span_with(|request: &Request<_>| {
    //                 let req_id = request
    //                     .headers()
    //                     .get("x-request-id")
    //                     .map(| v| v.to_str()
    //                         .unwrap_or(""))
    //                     .unwrap_or("");
    //                 info_span!(
    //                     "request",
    //                     method = %request.method(),
    //                     uri = %request.uri(),
    //                     reqid = ?req_id,
    //                 )
    //             })
    //             .on_request(DefaultOnRequest::new().level(Level::INFO))
    //             .on_response(DefaultOnResponse::new().level(Level::INFO)))
    //         .layer(PropagateRequestIdLayer::new(x_request_id))
    //         .layer(axum::auth_middleware::from_fn(log_request))
    //     );
    let api_route = Router::new().nest("/auth", auth_handler())
        .nest("/users", users_handler()
            .layer(middleware::from_fn(auth)))
        .nest("/", heath_handler())
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
