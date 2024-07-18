use std::sync::atomic::AtomicBool;
use std::sync::atomic::fence;
use std::sync::atomic::Ordering;

pub struct Mutex {
    flag: AtomicBool,
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex {
            flag: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // relaxed+acquire fence 来及时同步已发生的unlock (release)
        // cas时没有直接使用严格的acquire
        while self
            .flag
            .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {}
        fence(Ordering::Acquire);
    }

    pub fn unlock(&self) {
        self.flag.store(false, Ordering::Release);
    }
}