use crate::data::Request;
use anyhow::Context;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;


pub struct PoolProducer {
    inner: FutureProducer,
}

impl PoolProducer {
    pub fn new(brokers: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");
        Self { inner: producer }
    }

    pub async fn send(&self, request: Box<dyn Request>) -> anyhow::Result<(i32, i64)> {
        let topic = request.to_topic();
        let data = request.to_data();
        let message: FutureRecord<String, String> = FutureRecord::to(&topic).payload(&data);
        let (partition, offset) = self
            .inner
            .send(message, Duration::from_secs(0))
            .await
            .map_err(|err| anyhow::anyhow!("Producer error: {:?}", err))?;
        Ok((partition, offset))
    }
}
