//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::info;

#[tokio::main]
async fn main() {
    //! 如果你想在命令行中传递参数来获取值，你可以按照以下方式进行配置：
    //!
    //! 编译你的 Rust 程序，并在命令行中指定参数，例如：./your_program argument_value。
    //! 在这种情况下，env::args().nth(1) 将返回第一个参数的值作为 connect_addr
    let connect_addr =
        // env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
        "ws://127.0.0.1:8033";

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        // tx.unbounded_send(Message::binary(buf)).unwrap();
        let text = String::from_utf8_lossy(&buf).into_owned();
        info!("接收到命令行的输入：{}", &text);
        tx.unbounded_send(Message::text(text)).unwrap();
    }
}