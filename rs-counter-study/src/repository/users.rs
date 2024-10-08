use sqlx::Error;
use uuid::Uuid;

use crate::models::model::{LivingLabUser, LivingLabUsersRole};

pub trait UserExt {

    async fn get_user(&self,
                      user_id: Option<&str>,
                      name: Option<&str>,
                      email: Option<&str>,
                      token: Option<&str>,
    ) -> Result<Vec<LivingLabUser>, Error>;

    async fn get_user_by_user_id(&self,
                      user_id: Option<&str>,
    ) -> Result<LivingLabUser, Error>;

    async fn get_user_by_openid(&self, openid: Option<&str>, session_key: Option<&str>) -> Result<LivingLabUser, Error>;

    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<LivingLabUser>, Error>;

    async fn save_user<T: Into<String> + Send>(&self,
                                               name: T,
                                               email: T,
                                               password: T,
                                               verification_token: T,
                                               // token_expires_at: DateTime<Utc>
    ) -> Result<LivingLabUser, Error>;
    async fn get_user_count(&self) -> Result<i64, Error>;

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: String,
        name: T,
    ) -> Result<LivingLabUser, Error>;

    async fn update_user_role(&self,
                              user_id: Uuid,
                              name: LivingLabUsersRole, ) -> Result<LivingLabUser, Error>;

    async fn update_user_password(&self,
                                  user_id: Uuid,
                                  password: String) -> Result<LivingLabUser, Error>;
    // fn verified_token(&self, token: &str) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    //
    // async fn add_verified_token(&self, user_id: Uuid, token: &str, expires_at: Date) -> Result<(), Error>;
}
