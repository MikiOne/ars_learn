use std::thread;
use std::time::Duration;

use futures::prelude::*;

#[tokio::main]
async fn main() {
    let mut st = stream::iter(1..10)
        .filter(|x| future::ready(x % 2 == 0))
        .map(|x| {
            thread::sleep(Duration::from_millis(300));
            x * x
        });

    while let Some(x) = st.next().await {
        println!("Got item: {}", x);
    }
}