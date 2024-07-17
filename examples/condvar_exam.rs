#![allow(unused)]

use std::sync::Arc;

fn main() {
    wait_timeout();
}

fn wait() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    // let pair2 = pair.clone();

    // 在我们的锁内部，spawn 有一个新线程，然后等待它启动。
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        thread::sleep(std::time::Duration::from_millis(3000));
        // 我们通知 condvar 值已更改。
        cvar.notify_one();
    });

    // 等待线程启动。
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("started {:?}", started);
    while !*started {
        started = cvar.wait(started).unwrap();
        println!("while started {:?}", started);
    }
}

fn wait_while() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;

    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut pending = lock.lock().unwrap();
        *pending = false;
        // 我们通知 condvar 值已更改。
        cvar.notify_one();
    });

    // 等待线程启动。
    let (lock, cvar) = &*pair;
    // 只要 `Mutex<bool>` 内部的值为 `true`，我们就等待。
    let _guard = cvar.wait_while(lock.lock().unwrap(), |pending| { *pending }).unwrap();
}

fn wait_timeout() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    use std::time::Duration;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        thread::sleep(Duration::from_millis(130));

        // 我们通知 condvar 值已更改。
        cvar.notify_one();
    });

    // 等待线程启动
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    // 只要 `Mutex<bool>` 内的值为 `false`，我们就等待
    loop {
        let result = cvar.wait_timeout(started, Duration::from_millis(10)).unwrap();
        // 10 毫秒已过去，或者值已更改！
        started = result.0;
        println!("started: {}", started);
        if *started == true {
            // 我们已收到通知，并且值已更新，我们可以离开。
            break;
        }
    }
}
