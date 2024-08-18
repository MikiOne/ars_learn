use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::proto::MessageBo;

mod proto;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("0.0.0.0:5278").await?;
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    // 向服务器发送数据
    // stream.send(Bytes::from("Hello world!")).await?;

    let msg = MessageBo::new("Hello world!".to_string());
    stream.send(msg.try_into()?).await?;

    // 接收从服务器返回的数据
    if let Some(Ok(data)) = stream.next().await {
        // println!("Got: {:?}", String::from_utf8_lossy(&data));

        let msg_bo: MessageBo = data.try_into()?;
        println!("Got: {:?}", msg_bo);
    }
    Ok(())
}
