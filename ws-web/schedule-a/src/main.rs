use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

pub struct TimerTask<F, Fut>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), anyhow::Error>> + Send + 'static,
{
    interval: Duration,
    task: Arc<F>,
}

impl<F, Fut> TimerTask<F, Fut>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), anyhow::Error>> + Send + 'static,
{
    pub fn new(interval: Duration, task: F) -> Self {
        Self {
            interval,
            task: Arc::new(task),
        }
    }

    pub async fn start(self) -> Result<(), anyhow::Error> {
        let mut interval = time::interval(self.interval);
        loop {
            interval.tick().await;

            let task = (self.task)();
            if let Err(e) = task.await {
                eprintln!("任务执行错误: {}", e);
                return Err(e.into());
            }
        }
    }
}

// use std::future::Future;
// use std::sync::Arc;

// use tokio::time::{self, Duration};

// pub struct TimerTask<Fut>
// where
//     Fut: Future<Output=Result<(), anyhow::Error>> + Send + 'static,
// {
//     interval: Duration,
//     task: Arc<dyn Fn() -> Fut + Send + Sync + 'static>,
// }

// impl<Fut> TimerTask<Fut>
// where
//     Fut: Future<Output=Result<(), anyhow::Error>> + Send + 'static,
// {
//     pub fn new<F>(interval: Duration, task: F) -> Self
//     where
//         F: Fn() -> Fut + Send + Sync + 'static,
//     {
//         TimerTask {
//             interval,
//             task: Arc::new(task),
//         }
//     }

//     pub async fn start(&self) {
//         let mut interval = time::interval(self.interval);

//         loop {
//             interval.tick().await;
//             let task = (self.task)();
//             tokio::spawn(task); // 异步执行任务
//         }
//     }
// }


#[tokio::main]
async fn main() {
    // 创建一个定时任务，每10秒执行一次
    TimerTask::new(Duration::from_secs(10), || async_task()).start().await.expect("TimerTask execution failed");
}
async fn async_task() -> anyhow::Result<()> {
    println!("定时任务执行: 每10秒一次");
    // Ok(())
    Err(anyhow::anyhow!("定时任务执行失败"))
}