// use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
// use tracing::info;
//
// pub async fn init_database() -> Pool<Sqlite> {
//     match SqlitePoolOptions::new().connect("sqlite:data.db").await {
//         Ok(pool) => {
//             info!("数据库连接成功");
//             pool
//         }
//         Err(_) => {
//             info!("未找到数据库文件, 创建数据库文件并连接");
//             // 创建数据库文件
//             std::fs::File::create("./data.db").unwrap();
//             let pool = SqlitePoolOptions::new()
//                 .connect("sqlite:data.db")
//                 .await
//                 .unwrap();
//             info!("数据库连接成功");
//             pool
//         }
//     }
// }

use sqlx::{Error, PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::models::msg::Msg;

pub async fn init_database() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let database_url = "postgres://postgres:bix123456@localhost:5432/living_lab_base";
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    return pool;
}

#[derive(Debug, Clone)]
pub struct DBClient {
    pub(crate) pool: Pool<Postgres>,
}
impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }

    async fn save_message(&self, message: Msg) -> Result<(), Error> {
        sqlx::query("INSERT INTO messages
             (local_id, server_id, send_id, receiver_id, msg_type, content_type, content, send_time, platform)
             VALUES
             ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT DO NOTHING",)
            .bind(&message.local_id)
            .bind(&message.server_id)
            .bind(&message.send_id)
            .bind(&message.receiver_id)
            .bind(message.msg_type)
            .bind(message.content_type)
            .bind(&message.content)
            .bind(message.send_time)
            .bind(message.platform)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
