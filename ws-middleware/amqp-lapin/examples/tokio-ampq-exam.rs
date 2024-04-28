// use lapin::{Connection, ConnectionProperties, Result};
// use tokio::runtime::Runtime;
// use tokio_amqp::*;
//
// async fn tokio_main() -> Result<()> {
//     let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:admin123@127.0.0.1:5672/%2f".into());
//     let conn = Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await?; // Note the `with_tokio()` here
//     let channel = conn.create_channel().await?;
//
//     // Rest of your program
//     Ok(())
// }
//
// fn main() {
//     let rt = Runtime::new().expect("failed to create runtime");
//     rt.block_on(tokio_main()).expect("error");
// }