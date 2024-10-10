// use axum::extract::{Path, State};
// use axum::Json;
// use serde::Deserialize;
// use serde_json::{json, Value};
// use sqlx::PgPool;
// use crate::error::errors::HttpError;
// use crate::models::model::CounterRecord;
// use crate::routes::counter::get_user_counter;
// use crate::routes::jwt::Uid;
//
// #[derive(Debug, Deserialize)]
// pub struct AddPayload {
//     counter_id: i32,
//     step: i32,
// }
//
// pub async fn add_counter_record(State(pool): State<PgPool>,
//                                 Uid(user_id): Uid,
//                                 Json(payload): Json<AddPayload>, ) -> Result<Json<Value>, HttpError> {
//     let counter = get_user_counter(payload.counter_id, &user_id, &pool).await?;
//
//     let next_value = counter.value + payload.step;
//     sqlx::query(r#"insert into counter_records (counter_id, step, begin, end) values ($1, $2, $3, $4);
//     update counters set value = $5, updated_at = CURRENT_TIMESTAMP where id = $6"#, )
//         .bind(payload.counter_id)
//         .bind(payload.step)
//         .bind(counter.value)
//         .bind(next_value)
//         .bind(next_value)
//         .bind(payload.counter_id)
//         .execute(&pool)
//         .await?;
//     Ok((Json(json!({}))))
// }
//
// pub async fn list_counter_record(
//     Path(counter_id): Path<i32>,
//     State(pool): State<PgPool>,
//     Uid(user_id): Uid,
// ) -> Result<Json<Vec<CounterRecord>>, HttpError> {
//     let counter = get_user_counter(counter_id, &user_id, &pool).await?;
//
//     let records = sqlx::query_as::<_, CounterRecord>("select * from counter_records where counter_id = $1 order by id desc",)
//         .bind(counter_id)
//         .fetch_all(&pool).await?;
//     Ok(Json(records))
// }
