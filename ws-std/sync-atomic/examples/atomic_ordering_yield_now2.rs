use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Instant, Duration};

// 10个线程，每个线程100000次处理
const N_THREADS: usize = 10;
const N_TIMES: usize = 100000;

// R是待修改变量，SPIN_LOCK则是标记锁
static mut R: usize = 0;
static SPIN_LOCK: AtomicBool = AtomicBool::new(false);

// 全局存储线程对象
static mut THREADS: Vec<thread::Thread> = Vec::new();


fn main() {

    for t in 1..=10 {

        unsafe {
            R = 0;
        }

        let start_of_spin = Instant::now();

        let handles = (0..N_THREADS).map(|i| {
            thread::spawn(move || {
                unsafe {
                    for j in i * N_TIMES.. (i + 1) * N_TIMES {
                        while SPIN_LOCK.swap(true, Ordering::Relaxed) {
                            // 循环体内加一句出让线程
                            thread::yield_now();
                        }
                        R += j;
                        SPIN_LOCK.store(false, Ordering::Relaxed);

                        // 修改完成后唤醒所有线程
                        for thread in THREADS.iter() {
                            thread.unpark();
                        }
                    }
                }
            })
        }).collect::<Vec<_>>();

        unsafe {
            // 通过句柄克隆出对应的线程对象并装入全局数组
            THREADS = handles.iter().map(|handle| handle.thread().clone()).collect::<Vec<_>>();
            // 初始化结束后唤醒各个线程
            for thread in THREADS.iter() {
                thread.unpark();
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }


        let time_of_spin = start_of_spin.elapsed();
        let r = Arc::new(Mutex::new(0));
        let start_of_mutex = Instant::now();

        // 标准的多线程修改数据方法
        let handles = (0..N_THREADS).map(|i| {
            let r = r.clone();
            thread::spawn(move || {
                for j in i * N_TIMES.. (i + 1) * N_TIMES {
                    *r.lock().unwrap() += j;
                }
            })
        }).collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        let time_of_mutex = start_of_mutex.elapsed();

        println!("{t:3}: R = {}, r = {}, spin: {time_of_spin:?}, mutex: {time_of_mutex:?}", unsafe { R }, r.lock().unwrap());
    }
}