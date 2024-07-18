use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::anyhow;

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

struct Sender<T> {
    shared: Arc<Shared<T>>,
}

struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    fn send(&mut self, value: T) -> anyhow::Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receiver left"));
        }

        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let is_empty = inner.is_empty();
            inner.push_back(value);
            is_empty
        };

        if was_empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }

    fn total_queued_items(self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: Arc::clone(&self.shared)
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.senders.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> anyhow::Result<T> {
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(val) => return Ok(val),
                None if self.total_senders() == 0 => return Err(anyhow!("No sender left")),
                None => {
                    inner = self.shared.available.wait(inner)
                        .map_err(|_| anyhow!("lock poisoned"))?;
                }
            }
        }
    }
    fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T> Shared<T> {}

const INITIAL_SIZE: usize = 32;

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);

    let sender = Sender { shared: shared.clone() };
    let receiver = Receiver { shared };
    (sender, receiver)
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world".to_string()).unwrap();

        let result = r.recv().unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s1, mut r) = unbounded();
        let mut s2 = s1.clone();
        let mut s3 = s1.clone();

        let t1 = thread::spawn(move || {
            s1.send(1).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(2).unwrap();
        });
        let t3 = thread::spawn(move || {
            s3.send(3).unwrap();
        });
        for handler in [t1, t2, t3] {
            handler.join().unwrap();
        }

        let r1 = r.recv().unwrap();
        let r2 = r.recv().unwrap();
        let r3 = r.recv().unwrap();

        let mut res = [r1, r2, r3];
        res.sort();

        assert_eq!(res, [1, 2, 3]);
    }

    /// 当队列空的时候，receiver 所在的线程会被阻塞
    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();

        thread::spawn(move || {
            for (idx, val) in r.into_iter().enumerate() {
                assert_eq!(idx, val);
            }
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        thread::sleep(Duration::from_millis(1));

        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        thread::sleep(Duration::from_millis(1));
        assert_eq!(s1.total_queued_items(), 0);
    }

    // 如果现在所有 Sender 都退出作用域，Receiver 继续接收，到没有数据可读了
    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (mut s, mut r) = unbounded();
        let s1 = s.clone();

        let senders = [s, s1];
        let total = senders.len();

        for mut sender in senders {
            thread::spawn(move || {
                sender.send("helle").unwrap();
            }).join().unwrap();
        }

        for i in 0..total {
            r.recv().unwrap();
        }

        assert!(r.recv().is_err());
    }

    // 那么如果没有 Receiver了，Sender 发送时是不是也应该错误返
    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s, mut s1) = {
            let (mut s, _) = unbounded();
            let s1 = s.clone();
            (s, s1)
        };

        assert!(s.send("hello").is_err());
        assert!(s1.send("hello").is_err());
    }

    // 如果 Receiver 被阻塞，而此刻所有 Sender 都走了，那么 Receiver 就没有人唤醒，会带来资源的泄露。
    #[test]
    fn receiver_shall_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        let (mut sender, mut receiver) = unbounded::<usize>();

        let t1 = thread::spawn(move || {
            println!("t1");
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
        });

        thread::spawn(move || {
            let re = receiver.recv().unwrap();
            println!("t2 recv: {:?}", re);
            drop(s);
        });

        t1.join().unwrap();
    }
}