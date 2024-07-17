use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
}

struct Sender<T> {
    shared: Arc<Shared<T>>,
}

struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    fn send(&mut self, value: T) -> anyhow::Result<()> {
        let mut queue = self.shared.queue.lock().unwrap();
        queue.push_back(value);
        Ok(())
    }
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> anyhow::Result<Option<T>> {
        let mut queue = self.shared.queue.lock().unwrap();
        Ok(queue.pop_front())
    }
}

impl<T> Shared<T> {
    fn new() -> Self {
        Shared {
            queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        todo!()
    }
}

fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);

    let sender = Sender { shared: shared.clone() };
    let receiver = Receiver { shared: shared.clone() };
    (sender, receiver)
}


#[cfg(test)]
mod tests {
    use std::thread::Thread;
    use super::*;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world".to_string());

        let result = r.recv().unwrap();
        assert_eq!(result, Some("hello world".to_string()));
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s1, mut r) = unbounded();
        let s2 = s1.clone();
        let s3 = s1.clone();

        let t1 = Thread::spawn(move || {
            s1.send(1).unwrap();
        });
        let t2 = Thread::spawn(move || {
            s2.send(2).unwrap();
        });
        let t3 = Thread::spawn(move || {
            s3.send(3).unwrap();
        });
        for handler in [t1, t2, t3] {
            handler.join().unwrap();
        }

        let r1 = r.recv().unwrap();
        let r2 = r.recv().unwrap();
        let r3 = r.recv().unwrap();

        let res = [r1, r2, r3];
        res.sort();

        assert_eq!(res, [1, 2, 3]);
    }

    /// 当队列空的时候，receiver 所在的线程会被阻塞
    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, mut r) = unbounded();

    }
}