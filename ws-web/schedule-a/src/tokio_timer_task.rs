use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use log::{error, log};

pub struct TimerTask<F, Fut>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static,
{
    interval: Duration,
    task: Arc<F>,
}

impl<F, Fut> TimerTask<F, Fut>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static,
{
    pub fn new(interval: Duration, task: F) -> Self {
        Self {
            interval,
            task: Arc::new(task),
        }
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut interval = time::interval(self.interval);
        loop {
            interval.tick().await;

            let task = (self.task)();
            task.await;
            // match task.await {
            //     Ok(result) => match result {
            //         Ok(_) => {},
            //         Err(e) => {
            //             // 处理内部的错误
            //         }
            //     },
            //     Err(e) => {
            //         // 处理 await 本身的错误
            //         error!("任务执行错误: {:?}", e);
            //         return Err(Box::new(e));
            //     }
            // }
        }
    }
}

// 使用示例
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    async fn test_timer_task() {
        // 创建一个每3秒执行一次的定时任务
        let timer_task = TimerTask::new(Duration::from_secs(3), || async {
            println!("执行定时任务");
            Ok::<(), ()>(())
        });

        // 在后台运行定时任务
        tokio::spawn(async move {
            if let Err(e) = timer_task.start().await {
                eprintln!("定时任务出错: {:?}", e);
            }
        });

        // 让主线程等待10秒，以便观察定时任务的执行
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
