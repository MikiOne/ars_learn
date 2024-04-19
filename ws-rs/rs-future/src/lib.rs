use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

//! 学习复刻：https://www.bilibili.com/video/BV1Ki4y1C7gj
pub struct TimerFuture {
    // delay: Duration,
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程建共享状态
struct SharedState {
    /// 睡眠时间是否已经都过完
    completed: bool,
    /// ‘TimerFuture‘ 所运行于的任务的Waker
    /// 在设置‘completed=true‘之后，线程可以使用它来告诉
    /// ‘TimerFuture‘的任务可以唤醒，看到‘completed=true‘并前进
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // 检查计时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置 waker 以便当 timer 结束时线程可以唤醒当前任务，保证
            // Future 可以再次被 poll，并看到‘completed = true‘
            // 相比每次都克隆 waker，如果只做一次显然更有诱惑力
            // 但是‘TimerFuture‘、可在执行者的任务间移动，这会导致
            // 过期的 waker 指向错误的任务，从而阻止了‘TimerFuture‘
            // 正确的唤醒
            //
            // 注意:可以使用‘Waker::will_wake‘函数来检查这一点
            // 但为了简单起见，主我们就省略了这一点
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        // 创建一个新的 TimerFuture , 它将在提供的时限过后完成
        let shared_state = Arc::new(Mutex::new(SharedState { completed: false, waker: None }));

        // 生成新线程
        let thread_shared_state = shared_state.clone();
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 发出信号：计时器已停止唤醒 Future 被 poll的最后一个任务（如果存在的话）
            // 将 completed 设置为 true 并唤醒 waker
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
    // pub fn set_completed(&self) {
    //     let mut shared_state = self.shared_state.lock().unwrap();
    //     shared_state.completed = true;
    //     if let Some(waker) = shared_state.waker.take() {
    //         waker.wake();
    //     }
    // }
}
