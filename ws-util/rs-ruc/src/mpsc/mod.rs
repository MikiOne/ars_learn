use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use anyhow::anyhow;

mod tests;


const INIT_SIZE: usize = 32;

pub(crate) struct Counter {
    senders: AtomicUsize,
    receivers: AtomicUsize,
    available: std::sync::Condvar,
}

impl Default for Counter {
    fn default() -> Self {
        Counter {
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
            available: std::sync::Condvar::new(),
        }
    }
}

pub(crate) struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    counter: Counter,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Channel {
            queue: Mutex::new(VecDeque::with_capacity(INIT_SIZE)),
            counter: Counter::default(),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Channel<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.counter.senders.fetch_add(1, Ordering::Relaxed);
        Sender { shared: Arc::clone(&self.shared) }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.counter.senders.fetch_sub(1, Ordering::AcqRel);
        if old <= 0 {
            self.shared.counter.available.notify_all();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), anyhow::Error> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receiver left"));
        }

        let was_empty = {
            let mut queue = self.shared.queue.lock().unwrap();
            let is_empty = queue.is_empty();
            queue.push_back(value);
            is_empty
        };

        if was_empty {
            self.shared.counter.available.notify_one();
        }
        Ok(())
    }

    fn total_receivers(&self) -> usize {
        self.shared.counter.receivers.load(Ordering::SeqCst)
    }

    fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}


pub struct Receiver<T> {
    shared: Arc<Channel<T>>,
    // cached: VecDeque<T>,
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.counter.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T> Receiver<T> {
    fn recv(&self) -> Result<T, anyhow::Error> {
        // if self.total_senders() == 0 {
        //     return Err(anyhow!("no sender left"));
        // }

        let mut queue = self.shared.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(val) => {
                    return Ok(val);
                }
                None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
                None => {
                    queue = self.shared.counter.available.wait(queue).map_err(|_| anyhow!("condvar lock poisoned"))?;
                }
            }
        }
    }

    fn total_senders(&self) -> usize {
        self.shared.counter.senders.load(Ordering::SeqCst)
    }
}

fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel::default());
    let shared = Arc::clone(&channel);

    let sender = Sender { shared };
    let receiver = Receiver { shared: channel };

    (sender, receiver)
}