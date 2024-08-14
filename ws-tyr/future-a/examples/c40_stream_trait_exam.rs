use std::pin::Pin;
use std::task::{Context, Poll};

use futures::prelude::*;
use pin_project::pin_project;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader, Lines};

/// LineStream 内部使用 tokio::io::Lines
#[pin_project]
struct LineStream<R> {
    #[pin]
    lines: Lines<BufReader<R>>,
}

/// 从 BufReader 创建一个 LineStream
impl<R: AsyncRead> LineStream<R> {
    fn new(reader: BufReader<R>) -> Self {
        Self {
            lines: reader.lines()
        }
    }
}

/// 为 LineStream 实现 Stream trait
impl<R: AsyncRead> Stream for LineStream<R> {
    type Item = std::io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project()
            .lines
            .poll_next_line(cx)
            .map(Result::transpose)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = fs::File::open("./Cargo.toml").await?;
    let reader = BufReader::new(file);
    let mut st = LineStream::new(reader);
    while let Some(Ok(line)) = st.next().await {
        println!("Got: {}", line);
    }
    Ok(())
}
