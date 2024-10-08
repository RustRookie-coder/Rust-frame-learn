use amqp::Channel;
use redis::aio::MultiplexedConnection;
use crate::config::Config;
use crate::db::database::DBClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
    pub redis: MultiplexedConnection,
}
