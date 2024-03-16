use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let broker = "localhost:9092";
    let broker = "127.0.0.1:9092";
    let topic = "my-topic";

    // 创建生产者配置
    let mut client_config = ClientConfig::new();
    let producer_config = client_config.set("bootstrap.servers", broker)
        // .set("security.protocol", "SASL_PLAINTEXT")
        // .set("sasl.mechanisms", "PLAIN")
        // .set("sasl.username", "kafka")
        // .set("sasl.password", "kafka-secret")
    ;

    // 创建生产者
    let producer: FutureProducer = producer_config.create().expect("创建生产者失败");

    let message = "Hello, Kafka1111112222!";
    let record = FutureRecord::to(topic).key("").payload(message);

    let timeout = Timeout::After(Duration::from_secs(3));
    // 发送消息到主题
    match producer.send(record, timeout).await {
        Ok(delivery_result) => {
            let (partition, offset) = delivery_result;
            println!("消息发送成功，分区：{}，偏移量：{}", partition, offset);
        }
        Err(e) => {
            eprintln!("消息发送失败：{:?}", e);
        }
    }
}

// use std::thread;
// use std::time::Duration;
//
// // use clap::{value_t, App, Arg};
// use futures::stream::FuturesUnordered;
// use futures::{StreamExt, TryStreamExt};
// use log::info;
//
// use rdkafka::config::ClientConfig;
// use rdkafka::consumer::stream_consumer::StreamConsumer;
// use rdkafka::consumer::Consumer;
// use rdkafka::message::{BorrowedMessage, OwnedMessage};
// use rdkafka::producer::{FutureProducer, FutureRecord};
// use rdkafka::Message;
//
// async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
//     // Simulate some work that must be done in the same order as messages are
//     // received; i.e., before truly parallel processing can begin.
//     let arr = msg.payload().unwrap();
//     let s = String::from_utf8_lossy(arr);
//     println!("Message received: {}", s);
// }
//
// async fn record_owned_message_receipt(_msg: &OwnedMessage) {
//     // Like `record_borrowed_message_receipt`, but takes an `OwnedMessage`
//     // instead, as in a real-world use case  an `OwnedMessage` might be more
//     // convenient than a `BorrowedMessage`.
// }
//
// // Emulates an expensive, synchronous computation.
// fn expensive_computation<'a>(msg: OwnedMessage) -> String {
//     info!("Starting expensive computation on message {}", msg.offset());
//     thread::sleep(Duration::from_millis(rand::random::<u64>() % 5000));
//     info!(
//         "Expensive computation completed on message {}",
//         msg.offset()
//     );
//     match msg.payload_view::<str>() {
//         Some(Ok(payload)) => format!("Payload len for {} is {}", payload, payload.len()),
//         Some(Err(_)) => "Message payload is not a string".to_owned(),
//         None => "No payload".to_owned(),
//     }
// }
//
// // Creates all the resources and runs the event loop. The event loop will:
// //   1) receive a stream of messages from the `StreamConsumer`.
// //   2) filter out eventual Kafka errors.
// //   3) send the message to a thread pool for processing.
// //   4) produce the result to the output topic.
// // `tokio::spawn` is used to handle IO-bound tasks in parallel (e.g., producing
// // the messages), while `tokio::task::spawn_blocking` is used to handle the
// // simulated CPU-bound task.
// async fn run_async_processor(
//     brokers: String,
//     group_id: String,
//     input_topic: String,
//     output_topic: String,
// ) {
//     println!("Starting");
//     // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
//     let consumer: StreamConsumer = ClientConfig::new()
//         .set("group.id", &group_id)
//         .set("bootstrap.servers", &brokers)
//         .set("enable.partition.eof", "false")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "false")
//         .create()
//         .expect("Consumer creation failed");
//
//     consumer
//         .subscribe(&[&input_topic])
//         .expect("Can't subscribe to specified topic");
//
//     // Create the `FutureProducer` to produce asynchronously.
//     let producer: FutureProducer = ClientConfig::new()
//         .set("bootstrap.servers", &brokers)
//         .set("message.timeout.ms", "5000")
//         .create()
//         .expect("Producer creation error");
//
//     // Create the outer pipeline on the message stream.
//     let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
//         let producer = producer.clone();
//         let output_topic = output_topic.to_string();
//         println!("output_topic: {}", output_topic);
//         println!("borrowed_message: {:?}", borrowed_message);
//         async move {
//             // Process each message
//             record_borrowed_message_receipt(&borrowed_message).await;
//             // Borrowed messages can't outlive the consumer they are received from, so they need to
//             // be owned in order to be sent to a separate thread.
//             let owned_message = borrowed_message.detach();
//             record_owned_message_receipt(&owned_message).await;
//             tokio::spawn(async move {
//                 // The body of this block will be executed on the main thread pool,
//                 // but we perform `expensive_computation` on a separate thread pool
//                 // for CPU-intensive tasks via `tokio::task::spawn_blocking`.
//                 let computation_result =
//                     tokio::task::spawn_blocking(|| expensive_computation(owned_message))
//                         .await
//                         .expect("failed to wait for expensive computation");
//                 // let produce_future = producer.send(
//                 //     FutureRecord::to(&output_topic)
//                 //         .key("some key")
//                 //         .payload(&computation_result),
//                 //     Duration::from_secs(0),
//                 // );
//                 // match produce_future.await {
//                 //     Ok(delivery) => println!("Sent: {:?}", delivery),
//                 //     Err((e, _)) => println!("Error: {:?}", e),
//                 // }
//             });
//             Ok(())
//         }
//     });
//
//     info!("Starting event loop");
//     stream_processor.await.expect("stream processing failed");
//     info!("Stream processing terminated");
// }

// #[tokio::main]
// async fn main() {
//     // let brokers = "192.168.21.125:30090,192.168.21.125:30091,192.168.21.125:30092";
//     let brokers = "172.0.0.1:9092";
//     let group_id = "test";
//     let input_topic = "my-topic";
//     let output_topic = "docs";
//     let num_workers = 2;
//
//     (0..num_workers)
//         .map(|_| {
//             tokio::spawn(run_async_processor(
//                 brokers.to_owned(),
//                 group_id.to_owned(),
//                 input_topic.to_owned(),
//                 output_topic.to_owned(),
//             ))
//         })
//         .collect::<FuturesUnordered<_>>()
//         .for_each(|_| async { () })
//         .await
// }