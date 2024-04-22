use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::{Mutex, Notify};

// 使用 tokio::sync::Notify 替代 std::sync::Mutex。
struct TimerState {
    completed: Arc<Notify>,
    is_completed: bool,
}

pub struct TimerFuture {
    state: Arc<Mutex<TimerState>>,
    duration: Duration,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            completed: Arc::new(Notify::new()),
            is_completed: false,
        }));

        let thread_state = state.clone();

        std::thread::spawn(async move || {
            tokio::time::sleep(duration).await;
            let mut timer_state_lock = thread_state.lock().await;
            timer_state_lock.is_completed = true;
            timer_state_lock.completed.notify_one();
        });

        TimerFuture { state, duration }
    }
}

// impl Future for TimerFuture {
//     type Output = ();
//
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let mut state = self.state.lock().unwrap();
//         if state.is_completed {
//             Poll::Ready(())
//         } else {
//             tokio::pin!(state.completed.notified());
//             if state.completed.notified().is_ready() {
//                 state.is_completed = true;
//                 Poll::Ready(())
//             } else {
//                 Poll::Pending
//             }
//         }
//     }
// }
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.is_completed {
            Poll::Ready(())
        } else {
            let notified_future = state.completed.notified();
            match notified_future.poll_unpin(cx) {
                Poll::Ready(_) => {
                    state.is_completed = true;
                    Poll::Ready(())
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokio_timer::TimerFuture;
    use std::time::Duration;

    #[tokio::test]
    async fn test() {
        // 创建3个不同时长的future
        let timers = vec![
            TimerFuture::new(Duration::from_secs(1)),
            TimerFuture::new(Duration::from_secs(2)),
            TimerFuture::new(Duration::from_secs(3)),
        ];

        let start = std::time::Instant::now();

        for (i, timer) in timers.into_iter().enumerate() {
            timer.await; // 等待每一个定时器完成
            println!("定时器 {} 完成@ {:?}", i + 1, start.elapsed());
        }
    }
}
