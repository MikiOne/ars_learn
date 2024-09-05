use tokio::time::Duration;

use timer::TimerTask;

mod timer;

#[tokio::main]
async fn main() {
    // 创建一个定时任务，每10秒执行一次
    let task = TimerTask::new(Duration::from_secs(10), || {
        println!("定时任务执行: 每10秒一次");
    });

    // 启动定时任务
    task.start().await;
}