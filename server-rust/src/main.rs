mod routes;
use std::sync::Arc;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use dotenv::dotenv;
use redis::aio::MultiplexedConnection;
use redis::{Client, RedisResult};
use tracing::info;
use routes::create_router;
use tower_http::trace::TraceLayer;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgPool, PgPoolOptions};
// use tokio::sync::Mutex;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use rs_counter_study::app_state::AppState;
use rs_counter_study::config::Config;
use rs_counter_study::db::{cache, database, mq};
use rs_counter_study::db::database::DBClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::registry().with(fmt::layer()).init();
    dotenv().ok();
    let pool = database::init_database().await;

    let channel = mq::init_message_queue().await;

    // run migrations
    //+++++++++++++++++++++++++++++++++
    // sqlx::migrate!("src/infrastructure/postgres/migrations")
    //     .run(&pgpool)
    //     .await
    //     .unwrap();
    //+++++++++++++++++++++++++++++++++

    // server configuration
    let config = Config::init();
    // database client connection
    let db_client = DBClient::new(pool);
    // redis client connection
    let conn = cache::init_redis_client().await.unwrap();

    let app_state = AppState {
        env: config.clone(),
        db_client,
        redis: conn
    };
    // build our application with a routes
    let app = create_router(Arc::new(app_state.clone()));

    println!("🚀 Server started successfully");
    // run our app with hyper, listening globally on port 3000
    let address = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let server = address + ":" + &port;
    let listener = tokio::net::TcpListener::bind(server).await.unwrap();
    info!("server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

