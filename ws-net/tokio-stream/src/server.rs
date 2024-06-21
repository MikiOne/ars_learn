use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use crate::proto::MessageBo;

mod proto;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpListener::bind("0.0.0.0:5278").await?;

    loop {
        let (stream, addr) = stream.accept().await?;
        println!("accepted: {:?}", addr);

        // LengthDelimitedCodec 默认 4 字节长度
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
        tokio::spawn(async move {
            // 接收到的消息会只包含消息主体（不包含长度）
            while let Some(Ok(data)) = stream.next().await {
                // println!("Got: {:?}", String::from_utf8_lossy(&data));

                let msg_bo: MessageBo = data.try_into().unwrap();
                println!("Got: {:?}", msg_bo);
                // 发送的消息也需要发送消息主体，不需要提供长度
                // Framed/LengthDelimitedCodec 会自动计算并添加
                // stream.send(Bytes::from("goodbye world!")).await.unwrap();

                let msg = MessageBo::new("goodbye world!".to_string());
                stream.send(msg.try_into().unwrap()).await.unwrap();
            }
        });
    }
}
