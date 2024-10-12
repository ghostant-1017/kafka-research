use kafka_research::data::{MockRequest, Request};
use kafka_research::producer::PoolProducer;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let brokers = "localhost:9092";
    produce(brokers).await;
}

async fn produce(brokers: &str) {
    let producer = PoolProducer::new(brokers);
    loop {
        let request: Box<dyn Request> = Box::new(MockRequest {});
        if let Err(err) = producer.send(request).await {
            eprintln!("Failed to send message: {}", err);
        }
        println!("Message sent successfully!");
        // sleep
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
