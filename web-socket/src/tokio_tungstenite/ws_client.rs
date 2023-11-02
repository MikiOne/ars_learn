use std::io::{BufRead, Write};
use std::{env, io, process};

use anyhow::Context;
use futures::pin_mut;
use futures_channel::mpsc;
use futures_channel::mpsc::UnboundedSender;
use futures_util::{future, StreamExt};
use tracing::{error, info};
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::Message;
use url::Url;
use tokio::task;

type WsResult = anyhow::Result<()>;

type StdinTx = UnboundedSender<Message>;

async fn read_stdin(stdin_tx: StdinTx) -> WsResult {
    let stdin = io::stdin();
    let mut buffer = String::new();

    while stdin.lock().read_line(&mut buffer)? > 0 {
        let input = buffer.trim().to_lowercase();
        info!("You entered: {}", input);

        if input == String::from("exit") || input == String::from("quit") {
            // 客户端执行了关闭
            let close_frame = CloseFrame {
                code: CloseCode::Normal,
                reason: "The client performed a shutdown".into(),
            };
            stdin_tx.unbounded_send(Message::Close(Some(close_frame)))?;
        } else {
            stdin_tx.unbounded_send(Message::text(input))?;
        }

        buffer.clear();
    }

    Ok(())
}

/// cargo run:
///
/// 启动方式：RUST_BACKTRACE=1 cargo run --bin ws-client "ws://127.0.0.1:8033"
///
/// bash:
///
/// ./ws-client "ws://127.0.0.1:8033"
#[tokio::main]
async fn main() -> WsResult {
    tracing_subscriber::fmt::init();
    let ws_ser_addr = env::args().nth(1).context("请在启动的时候指定WS服务地址")?;
    // let ws_ser_url = Url::parse(&ws_ser_addr)?;
    let ws_ser_url = match Url::parse(&ws_ser_addr) {
        Ok(url) => url,
        Err(err) => {
            // error!("解析URL是出错: {:?}", err);
            panic!("解析URL是出错: {:?}", err);
        }
    };

    let (ws_stream, _) = tokio_tungstenite::connect_async(ws_ser_url).await?;
    info!("客户端已经连接到WS服务. server addr: {}", &ws_ser_addr);

    let (stdin_tx, stdin_rx) = mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (write, read) = ws_stream.split();
    let msg_write_to_ser = stdin_rx.map(Ok).forward(write);

    let ws_read_print = read.for_each(|message| async {
        info!("read.for_each message: {:?}", &message);
        let data = message.unwrap().into_data();
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        if let Err(err) = handle.write_all(&data) {
            error!("io::stdout write error: {:?}", err);
        }
    });

    pin_mut!(ws_read_print, msg_write_to_ser);
    future::select(ws_read_print, msg_write_to_ser).await;

    // Ok(())
    process::exit(0);
}
