use diesel::RunQueryDsl;
use sqlx::Error;
use uuid::Uuid;

use crate::db::database::DBClient;
use crate::models::model::{LivingLabUser, LivingLabUsersRole};
use crate::repository::users::UserExt;

impl UserExt for DBClient {
    async fn get_user(&self, user_id: Option<&str>, name: Option<&str>, email: Option<&str>, token: Option<&str>) -> Result<Vec<LivingLabUser>, Error> {
        todo!()
    }

    async fn get_user_by_user_id(&self, user_id: Option<&str>) -> Result<LivingLabUser, Error> {
        let user = sqlx::query_as::<_, LivingLabUser>
            (r#"SELECT id, user_id, openid, session_key, name, email, password, created_at, updated_at, verification_token FROM living_lab_users WHERE user_id = $1"#)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user_by_openid(&self, openid: Option<&str>, session_key: Option<&str>) -> Result<LivingLabUser, Error> {
        let user = sqlx::query_as::<_, LivingLabUser>
            (r#"SELECT id, user_id, openid, session_key, name, email, password, verification_token, created_at, updated_at FROM living_lab_users WHERE openid = $1"#)
            .bind(openid)
            .fetch_one(&self.pool)
            .await;

        let user: LivingLabUser = match user {
            Ok(user) => user,
            Err(Error::RowNotFound) => {
                let user_id = String::from(Uuid::new_v4());
                sqlx::query("insert into living_lab_users (user_id, openid, session_key, name, email, password, verification_token) values ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(&user_id)
                    .bind(openid)
                    .bind(session_key)
                    .bind("")
                    .bind("")
                    .bind("")
                    .bind("")
                    .execute(&self.pool)
                    .await?;
                sqlx::query_as::<_, LivingLabUser>
                    (r#"SELECT id, user_id, openid, session_key, name, email, password, verification_token, created_at, updated_at FROM living_lab_users WHERE openid = $1"#)
                    .bind(openid)
                    .fetch_one(&self.pool)
                    .await?
            }
            Err(e) => return Err(Error::from(e)),
        };
        Ok(user)
    }

    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<LivingLabUser>, Error> {
        let offset = (page - 1) * limit as u32;
        let users = sqlx::query_as::<_, LivingLabUser>
            (r#"SELECT id, user_id, openid, session_key, name, email, password, verification_token, created_at, updated_at FROM living_lab_users ORDER BY created_at DESC LIMIT $1 OFFSET $2"#)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    async fn save_user<T: Into<String> + Send>(&self, name: T, email: T, password: T, verification_token: T) -> Result<LivingLabUser, Error> {
        todo!()
    }

    async fn get_user_count(&self) -> Result<i64, Error> {
        let count = sqlx::query_scalar
            (r#"SELECT COUNT(*) FROM living_lab_users"#)
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    async fn update_user_name<T: Into<String> + Send>(&self, user_id: String, name: T) -> Result<LivingLabUser, Error> {
        sqlx::query("UPDATE living_lab_users
            SET name = $1, updated_at = Now()
            WHERE user_id = $2
            RETURNING name, email")
            .bind(name.into())
            .bind(&user_id)
            .execute(&self.pool)
            .await?;

        let user = self.get_user_by_user_id(Option::from(user_id.clone().as_str())).await?;
        Ok(user)
    }

    async fn update_user_role(&self, user_id: Uuid, name: LivingLabUsersRole) -> Result<LivingLabUser, Error> {
        todo!()
    }

    async fn update_user_password(&self, user_id: Uuid, password: String) -> Result<LivingLabUser, Error> {
        todo!()
    }
}
