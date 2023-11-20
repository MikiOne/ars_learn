use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let clone_pare = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*clone_pare;
        let mut started = lock.lock().unwrap();

        thread::sleep(Duration::from_secs(3));

        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one()
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("started: {}", &started); // false

    while !*started {
        println!("while>>>>>>>>>>>>");
        started = cvar.wait(started).unwrap();
        println!("while started: {}", &started); // true
    }
}
