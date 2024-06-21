use crossbeam::queue::ArrayQueue;
use std::thread;
use std::sync::Arc;

// 这是我们的简单任务类型，你可以根据你的需求来修改它
type Task = Box<dyn FnOnce() + Send + 'static>;

struct Scheduler {
    queue: Arc<ArrayQueue<Task>>,
}

impl Scheduler {
    fn new(capacity: usize) -> Self {
        Scheduler {
            queue: Arc::new(ArrayQueue::new(capacity)),
        }
    }

    fn spawn(&self, task: Task) {
        let queue = self.queue.clone();
        queue.push(task);
    }

    fn run(&self) {
        while let Ok(task) = self.queue.pop() {
            task();
        }
    }
}

fn main() {
    let scheduler = Scheduler::new(100);

    scheduler.spawn(Box::new(|| println!("hello")));
    scheduler.spawn(Box::new(|| println!("world")));

    let other_scheduler = scheduler.clone();
    thread::spawn(move || other_scheduler.run());

    scheduler.run();

    thread::sleep(std::time::Duration::from_secs(1));
}