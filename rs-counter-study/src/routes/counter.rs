// use std::ptr::read;
// use axum::extract::{Path, State};
// use axum::Json;
// use diesel::RunQueryDsl;
// use serde_json::{json, Value};
// use sqlx::{PgPool, Pool};
// use crate::error::errors::HttpError;
// use crate::models::model::{AddCounter, Counter, UpdateCounter};
// use crate::routes::jwt::Uid;
//
// pub async fn list(
//     State(pool): State<PgPool>,
//     Uid(user_id): Uid,
// ) -> Result<Json<Vec<Counter>>, HttpError> {
//     // user id
//     let counters = sqlx::query_as::<_, Counter>("select id, user_id, name, value, step, input_step, sequence from counters where user_id = $1 order by sequence desc")
//         .bind(&user_id)
//         .fetch_all(&pool)
//         .await;
//
//     let counters = match counters {
//         Ok(counters) => counters,
//         Err(e) => return Err(HttpError::from(e))
//     };
//     Ok(Json(counters))
// }
//
// pub async fn add(
//     State(pool): State<PgPool>,
//     Uid(user_id): Uid,
//     Json(payload): Json<AddCounter>,
// ) -> Result<Json<Value>, HttpError> {
//     let sequence = sqlx::query_as::<_, (i32, )>(
//         "select sequence from counters where user_id = $1 order by sequence desc limit 1"
//     ).bind(&user_id).fetch_one(&pool).await;
//
//     let sequence = match sequence {
//         Ok((sequence, )) => sequence + 1,
//         Err(sqlx::Error::RowNotFound) => 0,
//         Err(e) => return Err(e)
//     };
//
//     sqlx::query("insert into counters (user_id, name, value, step, input_step, sequence) values ($1, $2, $3, $4, $5, $6)")
//         .bind(&user_id)
//         .bind(payload.name)
//         .bind(payload.value)
//         .bind(payload.step)
//         .bind(if payload.input_step {1i32} else { 0i32 })
//         .bind(sequence)
//         .execute(&pool).await?;
//     Ok((Json(json!({}))))
// }
//
// pub async fn show(
//     State(pool): State<PgPool>,
//     Uid(user_id): Uid,
//     Path(id): Path<i32>,
// ) -> Result<Json<Counter>, HttpError> {
//     let counter = get_user_counter(id, &user_id, &pool).await?;
//     Ok(Json(counter))
// }
//
// pub async fn get_user_counter(id: i32, user_id: &String, pool: &PgPool) -> Result<Counter, HttpError> {
//     sqlx::query_as::<_, Counter>("select * from counter where id = $1 and user_id = $2")
//         .bind(id)
//         .bind(user_id)
//         .fetch_one(pool)
//         .await.map_err(|e| match e {
//         sqlx::Error::RowNotFound => HttpError::NotFound,
//         // _ => HttpError::from(e)
//     })
// }
//
// pub async fn update(State(pool): State<PgPool>,
//                     Uid(user_id): Uid,
//                     Path(id): Path<i32>, Json(payload): Json<UpdateCounter>,
// ) -> Result<Json<Value>, HttpError> {
//     get_user_counter(id, &user_id, &pool).await?;
//     sqlx::query("update counters set name = $1, step = $2, input_step = $3, updated_at = CURRENT_TIMESTAMP where id = $4")
//         .bind(payload.name)
//         .bind(payload.step)
//         .bind(if payload.input_step {1i32} else { 0i32 })
//         .bind(id)
//         .execute(&pool).await?;
//     Ok((Json(json!({}))))
// }
//
// pub async fn destroy(State(pool): State<PgPool>,
//                      Uid(user_id): Uid,
//                      Path(id): Path<i32>, Json(payload): Json<UpdateCounter>,
// ) -> Result<Json<Value>, HttpError> {
//     let counter = get_user_counter(id, &user_id, &pool).await?;
//     sqlx::query(r#"delete from counters where id = $1; delete from counter_records where counter_id = $2"#)
//         .bind(id)
//         .bind(id)
//         .execute(&pool).await?;
//     Ok((Json(json!({}))))
// }
//
// pub async fn top(State(pool): State<PgPool>,
//                  Uid(user_id): Uid,
//                  Path(id): Path<i32>, ) -> Result<Json<Value>, HttpError> {
//     let counter = get_user_counter(id, &user_id, &pool).await?;
//     let sequence = sqlx::query_as::<_, (i32, )>(
//         "select sequence from counters where user_id = $1 order by sequence desc limit 1"
//     ).bind(&user_id).fetch_one(&pool).await;
//
//     let sequence = match sequence {
//         Ok((sequence, )) => sequence + 1,
//         Err(e) => return Err(HttpError::from(e))
//     };
//     sqlx::query("update counters set sequence = $1, update_at = CURRENT_TIMESTAMP where id = $2")
//         .bind(sequence)
//         .bind(id)
//         .execute(&pool).await?;
//     Ok((Json(json!({}))))
// }
