use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("0.0.0.0:5278").await?;
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    // 向服务器发送数据
    stream.send(Bytes::from("Hello world!")).await?;

    // 接收从服务器返回的数据
    if let Some(Ok(data)) = stream.next().await {
        println!("Got: {:?}", String::from_utf8_lossy(&data));
    }
    Ok(())
}