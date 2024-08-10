use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tcp监听8080端口
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    // 开启loop
    loop {
        // 接收来自客户的的stream
        let (stream, addr) = listener.accept().await?;
        println!("Client address: {}", addr);

        // 新开一个tokio task
        tokio::spawn(async move {
            // 使用 LinesCodec 把 TCP 数据切成一行行字符串处理
            let framed = Framed::new(stream, LinesCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            // for循环读取reader中的每一行
            // for line in r.next().await {
            while let Some(line) = r.next().await {
                // 每读到一行就加个前缀发回
                w.send(format!("I got: {}", line?)).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}