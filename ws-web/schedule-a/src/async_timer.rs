use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() {
    let task = || tokio::spawn(async {
        println!("定时任务执行: 每10秒一次");
        // 执行异步操作...
    });

    // 创建一个定时任务，每10秒执行一次
    TimerTask::new(Duration::from_secs(10), task).start().await;
}

pub struct TimerTask {
    interval: Duration,
    task: Arc<dyn Fn() -> JoinHandle<()> + Send + Sync + 'static>,
}

impl TimerTask {
    pub fn new<F>(interval: Duration, task: F) -> Self
    where
        F: Fn() -> JoinHandle<()> + Send + Sync + 'static,
    {
        TimerTask {
            interval,
            task: Arc::new(task),
        }
    }

    pub async fn start(&self) {
        let mut interval = interval(self.interval);

        loop {
            interval.tick().await;
            (self.task)();
        }
    }
}