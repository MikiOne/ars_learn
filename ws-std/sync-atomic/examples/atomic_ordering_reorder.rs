use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};
fn main() {
    // 更严谨的测试可以用loom
    for _ in 0..100000 {
        acquire_release()
    }
}

fn acquire_release() {
    static FLAG: AtomicBool = AtomicBool::new(false);
    static mut DATA: u64 = 0;
    let a = thread::spawn(|| {
        unsafe { DATA = 42 };
        FLAG.store(true, Ordering::Relaxed);
        // 不允许有读写重排到store之后
    });
    let b = thread::spawn(|| {
        // 不允许有读写重排到load之前
        while !FLAG.load(Ordering::Relaxed) {}
        assert_eq!(unsafe { DATA }, 42);
    });

    a.join().unwrap();
    b.join().unwrap();
}