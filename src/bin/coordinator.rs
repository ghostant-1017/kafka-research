use rdkafka::consumer::{
    Consumer,
};
use std::fmt::Debug;
use std::future::Future;
use futures::StreamExt;
use rdkafka::Offset::Offset;
use rdkafka::{Message, TopicPartitionList};
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
    let topic = "pool-solution-epoch-1";
    let mut topic_partitions = 0;
    let metadata = consumer.inner.fetch_metadata(None, std::time::Duration::from_secs(1)).unwrap();
    for t in metadata.topics() {
        println!("Topic: {}", t.name());
        if t.name() == topic {
            topic_partitions = t.partitions().len();
            // break;
        }
    }
    if topic_partitions == 0 {
        return
    }

    println!("Topic: {}, partitions: {}", topic, topic_partitions);
    let mut tpl = TopicPartitionList::new();
    tpl.add_partition_range(topic, 0, (topic_partitions - 1) as i32);

    // ***获取消费的位置
    let committed_offsets = consumer.inner.committed_offsets(tpl, std::time::Duration::from_secs(10))
        .expect("Failed to get committed offsets");

    // ***获取高低水位
    for element in committed_offsets.elements() {
        let (lo, hi) = consumer.inner.fetch_watermarks(topic, element.partition(), std::time::Duration::from_secs(10)).expect("Failed to fetch watermarks");
        println!("Partition: {}, Committed offset: {:?}, metadata: {}, lo: {lo}, hi: {hi}", element.partition(), element.offset(), element.metadata());
    }

    // ***从某以offset开始消费
    let coordinator_gid = "coordinator";
    let coordinator = PoolConsumer::new(brokers, coordinator_gid);
    let mut tpl = TopicPartitionList::new();
    tpl.add_partition(topic, 0).set_offset(Offset(100)).expect("TODO: panic message");

    coordinator.inner.assign(&tpl).expect("Failed to assign partition");
    let mut stream = coordinator.inner.stream();

    loop {
        let message = stream.next().await.unwrap().unwrap();
        println!("Message: {}", message.offset());
    }
}

