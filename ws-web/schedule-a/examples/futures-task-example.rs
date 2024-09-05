use std::collections::BinaryHeap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

pub trait TimerTask: Future {
    fn name(&self) -> &'static str;
}

impl<T> TimerTask for T
where
    T: Future + 'static,
{
    fn name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
pub trait Timer {
    fn new() -> Self;
    fn set_interval(&mut self, duration: Duration);
    fn set_timeout(&mut self, duration: Duration);
    fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<()>;
}
struct TimerExecutor<T: Timer> {
    timer: Arc<Mutex<T>>,
    tasks: BinaryHeap<Arc<dyn TimerTask<Output=()>>>,
}

impl<T: Timer> TimerExecutor<T> {
    fn new() -> Self {
        TimerExecutor {
            timer: Arc::new(Mutex::new(T::new())),
            tasks: BinaryHeap::new(),
        }
    }

    fn spawn(&mut self, task: impl TimerTask) {
        self.tasks.push(Arc::new(task));
    }

    fn run(&mut self) {
        loop {
            let mut timer = self.timer.lock().unwrap();
            timer.set_interval(Duration::from_millis(100));

            while let Poll::Ready(_) = timer.poll_tick(&mut Context::from_waker(
                futures::task::noop_waker_ref(),
            )) {
                while let Some(task) = self.tasks.pop() {
                    if let Poll::Ready(_) = task.poll(&mut Context::from_waker(
                        futures::task::noop_waker_ref(),
                    )) {
                        println!("Task {} completed", task.name());
                    } else {
                        self.tasks.push(task);
                        break;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut executor = TimerExecutor::<SystemTimer>::new();

    executor.spawn(async {
        println!("Task 1 started");
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Task 1 completed");
    });

    executor.spawn(async {
        println!("Task 2 started");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Task 2 completed");
    });

    executor.run();
}