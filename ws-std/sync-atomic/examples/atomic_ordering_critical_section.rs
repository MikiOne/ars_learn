use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
fn main() {
    let lock = Arc::new(AtomicBool::new(false));
    let lock_clone_read = lock.clone();
    let lock_clone_store = lock.clone();

    thread::spawn(move || {
        // 持有锁
        lock.store(true, Ordering::SeqCst);
        // 执行临界区操作
    });

    // 等待锁被获取
    while !lock_clone_read.load(Ordering::Acquire) {}

    // 进入临界区，可以放心的执行临界区操作了
    println!("Critical section!");

    // 释放锁
    lock_clone_store.store(false, Ordering::Release);
}