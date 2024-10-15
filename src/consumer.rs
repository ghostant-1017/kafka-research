use anyhow::bail;
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use tracing::{info, warn};
use futures::StreamExt;
use tokio::task::JoinHandle;
use crate::data::Request;

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, re_balance: &Rebalance) {
        info!("Pre re_balance {:?}", re_balance);
    }

    fn post_rebalance(&self, re_balance: &Rebalance) {
        info!("Post re_balance {:?}", re_balance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

pub type LoggingConsumer = StreamConsumer<CustomContext>;

pub struct PoolConsumer {
    pub inner: LoggingConsumer,
}

impl PoolConsumer {
    pub fn new(brokers: &str, group_id: &str) -> Self {
        let consumer: LoggingConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "true")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(CustomContext)
            .expect("Consumer creation failed");

        Self {
            inner: consumer,
        }
    }

    pub fn subscribe(&self, epoch: u32) -> anyhow::Result<()> {
        let topic = format!("{}-{}", crate::SOLUTION_TOPIC_PREFIX, epoch);
        self.inner.subscribe(&[&topic])?;
        Ok(())
    }

    pub async fn start_process(&self) -> anyhow::Result<()> {
        let mut stream = self.inner.stream().ready_chunks(10);
        while let Some(messages) = stream.next().await {
            let mut handles = vec![];
            let mut is_end = false;
            for message in messages {
                match message {
                    Err(KafkaError::PartitionEOF(partition_id)) => {
                        info!("End of partition: {}", partition_id);
                        is_end = true;
                    },
                    Err(e) => {
                        bail!("Error while reading message: {:?}", e);
                    }
                    Ok(m) => {
                        info!("Received message: {:?}", m);
                        let handle = self.handle_message(m.detach());
                        handles.push(handle);
                    }
                }
            }
            let batch = handles.len();
            info!("Handle messages len: {}", handles.len());
            for handle in handles {
                let _ = handle.join().expect("join");
            }
            if batch > 0 {
                self.inner.commit_consumer_state(CommitMode::Sync)?;
            }

            if is_end {
                break;
            }
        }
        Ok(())
    }

    pub fn handle_message(&self, m: OwnedMessage) -> std::thread::JoinHandle<anyhow::Result<()>> {
        let handle = std::thread::spawn(move || {
            let payload = match m.payload_view::<str>() {
                None => "",
                Some(Ok(s)) => s,
                Some(Err(e)) => {
                    bail!("Error while deserializing message payload: {:?}", e);
                }
            };
            Ok(())
        });
        handle
    }
}
