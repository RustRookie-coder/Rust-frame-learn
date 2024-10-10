use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication, TopicResult};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::FutureProducer;
use tracing::log::info;
use crate::db::kafka::KafkaConfig;

pub struct ChatRpcService {
    kafka: FutureProducer,
    topic: String,
}

impl ChatRpcService {
    pub fn new(kafka: FutureProducer, topic: String) -> Self {
        Self {
            kafka,
            topic,
        }
    }

    pub async fn start(config: &KafkaConfig) {
        let broker = config.hosts.join(",");
        let producer: FutureProducer = ClientConfig::new().set("bootstrap.servers", &broker)
            .set("message.timeout.ms", config.producer.timeout.to_string(),)
            .set("socket.timeout.ms", config.connect_timeout.to_string())
            .set("acks", config.producer.acks.clone())
            // make sure the message is sent exactly once
            .set("enable.idempotence", "true")
            .set("retries", config.producer.max_retry.to_string())
            .set(
                "retry.backoff.ms",
                config.producer.retry_interval.to_string(),
            )
            .create()
            .expect("Producer creation error");


    }

    async fn ensure_topic_exists(
        topic_name: &str,
        brokers: &str,
        timeout: u16,
    ) -> Result<(), KafkaError> {
        // Create Kafka AdminClient
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("socket.timeout.ms", timeout.to_string())
            .create()?;

        // create topic
        let new_topics = [NewTopic {
            name: topic_name,
            num_partitions: 1,
            replication: TopicReplication::Fixed(1),
            config: vec![],
        }];

        /// fixme not find the way to check topic exist
        /// so just create it and judge the error
        /// but don't find the error type for topic exist
        /// and this way below can work well
        let options = AdminOptions::new();
        admin_client.create_topics(&new_topics, &options).await?;
        match admin_client.create_topics(&new_topics, &options).await {
            Ok(_) => {
                info!("Topic not exist; create '{}' ", topic_name);
                Ok(())
            }
            Err(KafkaError::AdminOpCreation(_)) => {
                println!("Topic '{}' already exists.", topic_name);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
