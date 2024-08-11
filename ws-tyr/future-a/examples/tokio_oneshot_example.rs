use tokio::sync::oneshot;
use tokio::time::{interval, sleep, Duration};

#[tokio::main]
async fn main() {
    send_on_drop().await
}
struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}
async fn send_on_drop() {
    let (send, recv) = oneshot::channel();

    let send_on_drop = SendOnDrop { sender: Some(send) };
    drop(send_on_drop);

    assert_eq!(recv.await, Ok("I got dropped!"));
}

async fn loop_tokio_select() {
    let (send, mut recv) = oneshot::channel();
    let mut interval = interval(Duration::from_millis(100));

    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        send.send("shut down").unwrap();
    });

    loop {
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut recv => {
                println!("Got message: {}", msg.unwrap());
                break;
            }
        }
    }
}

async fn drop_recv() {
    let (tx, rx) = oneshot::channel::<u32>();

    tokio::spawn(async move {
        drop(tx);
    });

    match rx.await {
        Ok(_) => panic!("This doesn't happen"),
        Err(_) => println!("the sender dropped"),
    }
}
async fn send_recv() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}