use std::thread;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<u8>();

    let sync_code = thread::spawn(move || {
        assert_eq!(Some(10), rx.blocking_recv());
    });

    let _ = tx.send(10);
    sync_code.join().unwrap();
}