use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt, thread};

struct Lock<T> {
    data: RefCell<T>,
    locked: AtomicBool,
}

unsafe impl<T> Sync for Lock<T> {}
impl<T> Debug for Lock<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Self { data: RefCell::new(data), locked: AtomicBool::new(false) }
    }

    fn lock(&self, op: impl FnOnce(&mut T)) {
        // 没有获取的锁，则spin
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {}

        // 执行
        op(&mut self.data.borrow_mut());

        // 释放锁
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|v| *v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v| *v *= 10);
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}
