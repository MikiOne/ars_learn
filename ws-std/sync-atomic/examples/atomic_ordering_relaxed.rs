use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn main() {
    let a = thread::spawn(|| {
        println!("thread a");
        let x = X.load(Ordering::Relaxed);
        Y.store(x, Ordering::Relaxed);
    });

    let b = thread::spawn(|| {
        println!("thread b");
        // 1
        let y = Y.load(Ordering::Relaxed);
        // 2
        X.store(42, Ordering::Relaxed);
    });

    a.join().unwrap();
    b.join().unwrap();

    assert_eq!(X.load(Ordering::Relaxed), 42);
    // 有可能, b线程的1和2不互相依赖，可以指令重排成2和1
    assert_eq!(Y.load(Ordering::Relaxed), 42);
}