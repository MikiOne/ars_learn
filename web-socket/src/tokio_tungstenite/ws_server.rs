use futures::channel::mpsc;
use futures::channel::mpsc::UnboundedSender;
use futures::{future, StreamExt, TryStreamExt};
use futures_util::SinkExt;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info, warn};

type WsResult<T> = anyhow::Result<T>;
type MsgTx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, MsgTx>>>;

const WS_ADDR: &str = "127.0.0.1:8033";

#[tokio::main]
async fn main() -> WsResult<()> {
    tracing_subscriber::fmt::init();
    let tcp_listener = TcpListener::bind(WS_ADDR).await?;
    info!("WebSocket listen on {}", WS_ADDR);

    let peers = PeerMap::new(Mutex::new(HashMap::new()));

    loop {
        let (tcp_stream, client_addr) = tcp_listener.accept().await?;
        info!("Client address: {}", &client_addr);
        tokio::spawn(handle_connection(tcp_stream, client_addr, peers.clone()));
    }
}

async fn handle_connection(
    tcp_stream: TcpStream,
    client_addr: SocketAddr,
    peers: PeerMap,
) -> WsResult<()> {
    let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await?;
    info!("Client [{}] WebSocket链接已建立", &client_addr);

    // 多个生产者，单个消费者
    let (msg_tx, msg_rx) = mpsc::unbounded();
    peers.lock().unwrap().insert(client_addr, msg_tx);

    let (write, read) = ws_stream.split();
    let broadcast_read_msg = read.try_for_each(|msg| {
        handle_message(msg, client_addr, peers.clone());
        future::ok(())
    });

    let rx_forward = msg_rx.map(Ok).forward(write);
    if let Err(err) = tokio::try_join!(broadcast_read_msg, rx_forward) {
        error!("write msg error: {:?}", err);
        peers.lock().unwrap().remove(&client_addr);
    }
    Ok(())
}

fn handle_message(msg: Message, client_addr: SocketAddr, peers: PeerMap) {
    let handle_text = |text| {
        info!("Receive text of msg: {}", &text);

        let peers = peers.lock().unwrap();
        let receivers = peers.iter().filter(|(addr, _)| addr != &&client_addr);
        for (addr, tx) in receivers {
            if tx.is_closed() {
                error!("address[{}] of sender is closed", addr);
                return;
            }
            // 向这个地址发送消息
            info!("Send message: address[{}] msg: {}", &addr, &text);
            if let Err(err) = tx.unbounded_send(msg.clone()) {
                error!("send message[{}] error: {:?}", msg, err);
            }
        }
    };

    match msg.clone() {
        Message::Text(text) => handle_text(text),
        Message::Close(cls) => {
            warn!("Client[{}] is close: {:?}", &client_addr, &cls);
            if let Some(msg_tx) = peers.lock().unwrap().remove(&client_addr) {
                let is_closed = msg_tx.is_closed();
                if !is_closed {
                    info!("drop msg_tx.is_closed(): {}", msg_tx.is_closed());
                    drop(msg_tx);
                    // let _ = msg_tx.unbounded_send(Message::Close(None));
                }
            }
        }
        _ => {
            error!("Unsupported message type");
            panic!("Unsupported message type")
        }
    }
}
