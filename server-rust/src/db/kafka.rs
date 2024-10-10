use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaConfig {
    pub hosts: Vec<String>,
    pub topic: String,
    pub connect_timeout: u16,
    pub group: String,
    pub producer: KafkaProducer,
    pub consumer: KafkaConsumer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaProducer {
    pub timeout: u16,
    pub acks: String,
    pub max_retry: u8,
    pub retry_interval: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaConsumer {
    pub session_timeout: u16,
    pub auto_offset_reset: String,
}
