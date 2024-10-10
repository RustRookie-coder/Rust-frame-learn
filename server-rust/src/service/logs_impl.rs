use sqlx::Error;

use crate::db::database::DBClient;
use crate::dto::dto::CreateLog;
use crate::repository::log::LogExt;

impl LogExt for DBClient {
    async fn create_access_log(&self, log: CreateLog) -> Result<u64, Error> {
        let res = sqlx::query
            ("INSERT INTO living_lab_access_logs (user_id, name, username, ip_addr, method, request_url, device, execution_time, error)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);")
            .bind(log.user_id.as_str())
            .bind(log.name.as_str())
            .bind(log.username.as_str())
            .bind(log.ip_addr)
            .bind(log.method.as_str())
            .bind(log.request_url.as_str())
            .bind(log.device.as_str())
            .bind(log.execution_time)
            .bind(log.error.as_str())
            .execute(&self.pool)
            .await.map_err(|e| Error::from(e));

        Ok(res?.rows_affected())
    }
}
