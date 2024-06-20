use std::sync::{Arc, Mutex};
use std::thread;

use crossbeam::sync::WaitGroup;
/// 文档：https://docs.rs/crossbeam/latest/crossbeam/sync/struct.WaitGroup.html
///
/// WaitGroup is very similar to Barrier, but there are a few differences:
///
/// Barrier needs to know the number of threads at construction, while WaitGroup is cloned to register more threads.
///
/// A Barrier can be reused even after all threads have synchronized, while a WaitGroup synchronizes threads only once.
///
/// All threads wait for others to reach the Barrier. With WaitGroup, each thread can choose to either wait for other threads or to continue without blocking.
fn main() {
    let num_threads = 10;
    let num = Arc::new(Mutex::new(0));

    // Create a new wait group.
    let wg = WaitGroup::new();

    // let mut handlers = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        // Create another reference to the wait group.
        let wgc = wg.clone();

        let numc = num.clone();

        thread::spawn(move || {
            let mut numc = numc.lock().unwrap();
            *numc += 1;
            println!("numc: {}", numc);

            // Drop the reference to the wait group.
            drop(wgc);
        });

        // handlers.push(thread::spawn(move || {
        //     let mut numc = numc.lock().unwrap();
        //     *numc += 1;
        //
        //     // Drop the reference to the wait group.
        //     drop(wgc);
        // }));
    }

    // for handler in handlers {
    //     handler.join().un
    // }

    // Block until all threads have finished their work.
    wg.wait();

    let result = {
        let result = num.clone();
        let res = *result.lock().unwrap();
        res
    };
    assert_eq!(result, num_threads);
}
