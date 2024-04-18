use futures::stream::StreamExt;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;

#[tokio::main]
async fn main() {
    // // 建立到 WebSocket 服务器的连接
    let (mut ws_stream, _) =
        tokio_tungstenite::connect_async("ws://localhost:8033").await.expect("Failed to connect");

    // 关闭 WebSocket 连接
    let close_frame = CloseFrame { code: CloseCode::Normal, reason: "23456".into() };
    ws_stream.close(Some(close_frame)).await.unwrap();

    let close_res = |result| async {
        println!("for_each result: {:?}", &result);
        if let Err(e) = result {
            eprintln!("Error closing WebSocket connection: {:?}", e);
        }
    };

    // 等待连接关闭
    ws_stream.take(1).for_each(close_res).await;
}
