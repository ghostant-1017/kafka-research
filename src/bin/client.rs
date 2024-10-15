use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{
    BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer,
};
use rdkafka::error::KafkaResult;
use rdkafka::message::Headers;
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use std::fmt::Debug;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
use kafka_research::consumer::PoolConsumer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    let brokers = "localhost:9092";
    let group_id = "verifier";
    let consumer = PoolConsumer::new(brokers, group_id);
    let mut epoch = 1;
    loop {
        info!("subscribe epoch {}", epoch);
        consumer.subscribe(epoch).unwrap();
        match consumer.start_process().await {
            Ok(_) => info!("Process finished epoch {} successfully", epoch),
            Err(err) => warn!("Process failed: {}", err),
        }
        epoch += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

}

