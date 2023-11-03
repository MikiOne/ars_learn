use futures_channel::mpsc;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let connect_addr =
        env::args().nth(1).unwrap_or_else(|| panic!("This program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = mpsc::unbounded();
    let (exit_tx, mut exit_rx) = mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx, exit_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = read.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);

    future::select(stdin_to_ws, ws_to_stdout).await;

    // 等待向服务器发送 "quit" 消息完成后再退出程序
    while let Some(_) = exit_rx.recv().await {}
}

async fn read_stdin(tx: mpsc::UnboundedSender<Message>, mut exit_tx: mpsc::UnboundedSender<()>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);

        if let Ok(input) = String::from_utf8(buf.clone()) {
            if input.trim() == "quit" {
                // 向服务器发送退出请求并通知主任务退出程序
                let _ = tx.unbounded_send(Message::text("quit".to_owned()));
                let _ = exit_tx.send(());
                return;
            }
        }

        let _ = tx.unbounded_send(Message::binary(buf));
    }
}
