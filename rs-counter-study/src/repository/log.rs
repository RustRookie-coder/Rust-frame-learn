use sqlx::Error;
use crate::dto::dto::CreateLog;
use crate::models::model::LivingLabAccessLogs;

pub trait LogExt {
    fn create_access_log(&self, log: CreateLog) -> impl std::future::Future<Output = Result<u64, Error>> + Send;
}
