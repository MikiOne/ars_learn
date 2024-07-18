use std::sync::atomic::AtomicBool;

// 10个线程，每个线程100000次处理
const N_THREADS: usize = 10;
const N_TIMES: usize = 100000;

// R是待修改变量，SPIN_LOCK则是标记锁
static mut R: usize = 0;
static SPIN_LOCK: AtomicBool = AtomicBool::new(false);


fn main() {

}