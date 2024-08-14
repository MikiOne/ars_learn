use std::{thread, time};
use futures::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut st = stream::iter(1..10)
        .filter(|x| future::ready(x % 2 == 0))
        .map(|x| {
            thread::sleep(Duration::from_secs(1));
            x * x
        });

    while let Some(x) = st.next().await {
        println!("Got item: {}", x);
    }
}