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
use tracing::{info, warn};
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;

/// 如果你想在命令行中传递参数来获取值，你可以按照以下方式进行配置：
///
/// 编译你的 Rust 程序，并在命令行中指定参数，例如：./your_program argument_value。
///
/// 在这种情况下，env::args().nth(1) 将返回第一个参数的值作为 connect_addr
///
/// let connect_addr =
/// env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let connect_addr =
        // env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
        "ws://127.0.0.1:8033";

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    info!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    // match stdin_rx.try_next() {
    //     Ok(Some(msg)) => {
    //         if let Message::Close(cc) = msg {
    //             info!("Message::Close: {:?}", cc);
    //             process::exit(0);
    //         }
    //     }
    //     Ok(None) => {
    //         warn!("stdin_rx.try_next 其他类型消息跳过");
    //     }
    //     Err(err) => {
    //         error!("stdin_rx.try_next error: {:?}", err);
    //     }
    // }

    // tokio::spawn(async move {
    //     while let Ok(Some(Message::Close(cc))) = stdin_rx.try_next() {
    //         info!("Message::Close: {:?}", cc);
    //         process::exit(1);
    //     }
    // });

    // while let Ok(Some(Message::Close(cc))) = stdin_rx.try_next() {
    //     info!("Message::Close: {:?}", cc);
    //     process::exit(0);
    // }

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = read.for_each(|message| async {
        info!("read.for_each message: {:?}", &message);
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;

    warn!("process::exit<<<<<<<<<");
    // process::exit(0);
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
        // 获取到标准输入时，字符串后有回车符号‘\n’，需要trim后才能使用==做比较
        let text = text.trim_end_matches('\n');
        // let text = String::from_utf8(buf).unwrap();
        info!("接收到命令行的输入：{}", &text);

        // if text.as_str().eq_ignore_ascii_case("quit"){
        let quit = String::from("quit");
        let exit = String::from("exit");
        if quit.eq_ignore_ascii_case(text) || exit.eq_ignore_ascii_case(text) {
            let close_frame =
                CloseFrame { code: CloseCode::Normal, reason: "Client execute close".into() };
            tx.unbounded_send(Message::Close(Some(close_frame))).unwrap();
        } else {
            tx.unbounded_send(Message::text(text)).unwrap();
        }
    }
}
