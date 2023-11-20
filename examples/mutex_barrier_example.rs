use std::sync::{Arc, Barrier, Mutex};
use std::thread;

///文档：https://mp.weixin.qq.com/s/X4hIhzggiv-nv4vAxvX6Rw
///
///Rust 有很多种控制并发的方式，Barrier（屏障）是其中一种用来同步多线程计算的方式。
///
/// 今天拿代码来简单看下。
///
/// 比如我们要多线程计算，期望所有线程都计算完毕再输出最终结果。常规多线程代码示例可以用线程 join 来等待
fn share_with_mutex() {
    let num_threads: i32 = 10;
    let my_mutex = Arc::new(Mutex::new(0));
    let mut handlers = Vec::with_capacity(num_threads as usize);

    // saturating_sub()
    for _ in 0..num_threads {
        let my_lock = my_mutex.clone();
        handlers.push(thread::spawn(move || {
            let mut guard = my_lock.lock().unwrap();
            *guard += 1;
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    {
        let my_lock = my_mutex.clone();
        let result = *my_lock.lock().unwrap();
        assert_eq!(result, num_threads);
    }
}

/// 而如果用 Barrier，我们可以这么写：
fn share_with_barrier() {
    let num_threads = 10;
    let my_mutex = Arc::new(Mutex::new(0));
    let barrier = Arc::new(Barrier::new(num_threads + 1));

    for i in 0..num_threads {
        let my_lock = my_mutex.clone();
        let my_barr = barrier.clone();
        thread::spawn(move || {
            let mut guard = my_lock.lock().unwrap();
            *guard += 1;

            // Release the lock to prevent a deadlock
            drop(guard);
            println!("thread {} is ready", i);

            // Blocks the current thread until all threads have rendezvoused here.
            my_barr.wait();
            println!("thread {} is done", i);
        });
    }

    // A barrier will block `n`-1 threads which call [`wait()`] and then wake
    // up all threads at once when the `n`th thread calls [`wait()`].
    barrier.wait();

    // {
    //     let my_lock = my_mutex.clone();
    //     let result = *my_lock.lock().unwrap();
    //     assert_eq!(result, num_threads);
    // }
    let result = {
        let my_lock = my_mutex.clone();
        let x = *my_lock.lock().unwrap();
        x
    };
    assert_eq!(result, num_threads);
}

///Barrier 可以用 wait 来控制 n 个线程的同步，数量需要提前指明。当调用 wait 时，如果不是第 n 个，就会一直阻塞当前线程，直到第 n 个 wait 调用，才能进行后续操作。
///
/// 这种机制就像在多个线程中插入了一道屏障，当所有线程都执行到这里时，才能解除屏障继续向后执行。
///
/// 当然这样实现相较于第一种，在线程数量大的时候也是会有比较明显的性能开销的，底层是使用 condvar+mutex 来实现的。这种组合也是一种有意思的并发控制方式，下次我们再聊聊它们。
fn main() {
    share_with_barrier();
}
