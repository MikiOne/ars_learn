use std::sync::Arc;

use tokio::time::{self, Duration};

pub struct TimerTask {
    interval: Duration,
    task: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl TimerTask {
    pub fn new<F>(interval: Duration, task: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        TimerTask {
            interval,
            task: Arc::new(task),
        }
    }

    pub async fn start(&self) {
        let mut interval = time::interval(self.interval);

        loop {
            interval.tick().await;
            (self.task)();
        }
    }
}