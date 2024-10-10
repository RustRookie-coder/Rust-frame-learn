use std::fmt::Debug;
use std::sync::mpsc;
use axum::async_trait;
use crate::error::errors::HttpError;
use crate::models::msg::Msg;

#[async_trait]
pub trait MsgStoreRepo: Sync + Send + Debug {
    /// save message to db
    async fn save_message(&self, message: Msg) -> Result<(), HttpError>;

    /// delete message. need message structure
    async fn delete_message(&self, message_id: &str) -> Result<(), HttpError>;

    async fn delete_messages(&self, user_id: &str, msg_seq: Vec<i64>) -> Result<(), HttpError>;

    #[allow(dead_code)]
    async fn get_message(&self, message_id: &str) -> Result<Option<Msg>, HttpError>;

    /// need to think about how to get message from receive box,
    /// use stream or use pagination? prefer stream
    async fn get_messages_stream() -> Result<mpsc::Receiver<Msg>, HttpError>;

    async fn get_messages(&self, user_id: &str,send_start: i64, send_end:i64, rec_start: i64, rec_end: i64) -> Result<Vec<Msg>, HttpError>;

    async fn msg_read(&self, user_id: &str, msg_seq: &[i64]) -> Result<(), HttpError>;
}

pub trait MsgRecBoxCleaner: Sync + Send {
    /// run a task which use tokio to clean message receive box
    /// clean all messages except the messages that type is group operations related
    /// #params
    /// *period: the period of time to clean, unit is day
    /// *types: the types of messages to not clean, such as group operations related use msgType
    fn clean_receive_box(&self, period: i64, types: Vec<i32>);
}
