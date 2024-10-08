use redis::aio::MultiplexedConnection;
use redis::{Client, RedisResult};

pub async fn init_redis_client() -> RedisResult<MultiplexedConnection> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url);
    let client = match client {
        Ok(client) => { client }
        Err(err) => {
            println!("🔥 Failed to connect to the redis server database: {:?}", err);
            std::process::exit(1);
        }
    };
    let conn = client.get_multiplexed_tokio_connection().await.expect("🔥 Failed to connect to the database");
    Ok(conn)
}
