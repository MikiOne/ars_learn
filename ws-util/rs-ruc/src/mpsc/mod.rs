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
    channel: Mutex<VecDeque<T>>,
    counter: Counter,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Channel {
            channel: Mutex::new(VecDeque::with_capacity(INIT_SIZE)),
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
        Sender { shared: self.shared.clone() }
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
        let mut queue = self.shared.channel.lock().unwrap();
        let receivers = self.shared.counter.receivers.load(Ordering::SeqCst);
        if receivers == 0 {
            return Err(anyhow!("no receiver left"));
        }

        queue.push_back(value);
        // if self.channel.counter == 1 {
        //     self.channel.counter.available.notify_one();
        // }
        Ok(())
    }
}


pub struct Receiver<T> {
    shared: Arc<Channel<T>>,
    // cached: VecDeque<T>,
}

impl<T> Receiver<T> {
    fn recv(&self) -> Result<T, anyhow::Error> {
        let mut queue = self.shared.channel.lock().unwrap();
        if self.total_senders() == 0 {
            return Err(anyhow!("no sender left"));
        }

        loop {
            match queue.pop_front() {
                Some(val) => {
                    return Ok(val);
                }
                None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
                None => {
                    queue = self.shared.channel.wait(queue).unwrap();
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
    let sender = Sender { shared: channel.clone() };
    let receiver = Receiver { shared: channel };
    (sender, receiver)
}