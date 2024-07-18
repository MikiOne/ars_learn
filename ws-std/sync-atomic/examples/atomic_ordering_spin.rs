/// - Rust进阶: 关于atomic原子操作，ordering指令顺序，spin自旋锁的一些实验:
/// https://mp.weixin.qq.com/s/2-eRN2LhW5r9iFLGi1zcOw
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

// 10个线程，每个线程100000次处理
const N_THREADS: usize = 10;
const N_TIMES: usize = 100000;

// R是待修改变量，SPIN_LOCK则是标记锁
static mut R: usize = 0;
static SPIN_LOCK: AtomicBool = AtomicBool::new(false);


fn main() {
    for t in 1..=10 {
        unsafe {
            R = 0;
        }

        let start_of_spin = Instant::now();
        let handles = (0..N_THREADS).map(|i| {
            thread::spawn(move || {
                unsafe {
                    for j in i * N_TIMES..(i + 1) * N_TIMES {
                        // 用 while 循环来阻塞线程，swap 来保证判断和修改的原子性，此处用了最宽松的Relaxed
                        while SPIN_LOCK.swap(true, Ordering::SeqCst) {}
                        // 修改数据，本身并不是线程安全的
                        R += j;
                        // 把锁改回 false 让所有线程继续抢锁
                        SPIN_LOCK.store(false, Ordering::SeqCst);
                    }
                }
            })
        }).collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }


        let time_of_spin = start_of_spin.elapsed();
        // 创建 Mutex 锁
        let r = Arc::new(Mutex::new(0));
        let start_of_mutex = Instant::now();

        // 标准的多线程修改数据方法
        let handles = (0..N_THREADS).map(|i| {
            let r = r.clone();
            thread::spawn(move || {
                for j in i * N_TIMES..(i + 1) * N_TIMES {
                    // 加锁修改数据
                    *r.lock().unwrap() += j;
                }
            })
        }).collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        let time_of_mutex = start_of_mutex.elapsed();

        println!("{t:2}: R = {}, r = {}, spin: {time_of_spin:?}, mutex: {time_of_mutex:?}", unsafe { R }, r.lock().unwrap());
    }
}
// Ordering::Relaxed
//  1: R = 499587643566, r = 499999500000, spin: 349.579875ms, mutex: 217.0715ms
//  2: R = 499917116546, r = 499999500000, spin: 336.923917ms, mutex: 222.60625ms
//  3: R = 499887978980, r = 499999500000, spin: 398.956417ms, mutex: 222.627042ms
//  4: R = 499839625729, r = 499999500000, spin: 450.148ms, mutex: 210.622042ms
//  5: R = 499957909074, r = 499999500000, spin: 345.002041ms, mutex: 203.559625ms
//  6: R = 499868065448, r = 499999500000, spin: 417.196542ms, mutex: 194.96825ms
//  7: R = 499959848435, r = 499999500000, spin: 356.47375ms, mutex: 200.237708ms
//  8: R = 499629443075, r = 499999500000, spin: 354.409458ms, mutex: 217.245375ms
//  9: R = 499971693036, r = 499999500000, spin: 323.264625ms, mutex: 186.566917ms
// 10: R = 499695860932, r = 499999500000, spin: 332.170667ms, mutex: 223.760083ms

// while SPIN_LOCK.swap(true, Ordering::Acquire) { }
// SPIN_LOCK.store(false, Ordering::Release);
//  1: R = 499999500000, r = 499999500000, spin: 422.424416ms, mutex: 223.7ms
//  2: R = 499999500000, r = 499999500000, spin: 416.173958ms, mutex: 201.668584ms
//  3: R = 499999500000, r = 499999500000, spin: 376.510875ms, mutex: 210.613292ms
//  4: R = 499999500000, r = 499999500000, spin: 381.683584ms, mutex: 217.172417ms
//  5: R = 499999500000, r = 499999500000, spin: 416.417459ms, mutex: 214.1995ms
//  6: R = 499999500000, r = 499999500000, spin: 396.769083ms, mutex: 224.328333ms
//  7: R = 499999500000, r = 499999500000, spin: 411.172708ms, mutex: 187.797833ms
//  8: R = 499999500000, r = 499999500000, spin: 394.088292ms, mutex: 210.824375ms
//  9: R = 499999500000, r = 499999500000, spin: 422.949375ms, mutex: 211.917375ms
// 10: R = 499999500000, r = 499999500000, spin: 438.827291ms, mutex: 223.047833ms

// Ordering::SeqCst
//  1: R = 499999500000, r = 499999500000, spin: 404.2365ms, mutex: 204.253166ms
//  2: R = 499999500000, r = 499999500000, spin: 413.544416ms, mutex: 190.305709ms
//  3: R = 499999500000, r = 499999500000, spin: 422.824541ms, mutex: 208.189041ms
//  4: R = 499999500000, r = 499999500000, spin: 443.690375ms, mutex: 229.907584ms
//  5: R = 499999500000, r = 499999500000, spin: 419.383208ms, mutex: 232.443042ms
//  6: R = 499999500000, r = 499999500000, spin: 415.064208ms, mutex: 196.281167ms
//  7: R = 499999500000, r = 499999500000, spin: 462.073542ms, mutex: 201.398333ms
//  8: R = 499999500000, r = 499999500000, spin: 459.241458ms, mutex: 216.79525ms
//  9: R = 499999500000, r = 499999500000, spin: 392.984916ms, mutex: 237.290375ms
// 10: R = 499999500000, r = 499999500000, spin: 421.238209ms, mutex: 205.499041ms