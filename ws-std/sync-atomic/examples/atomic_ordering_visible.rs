use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread,
};

static S: AtomicU64 = AtomicU64::new(0);
fn relaxed(i: i32) {
    let a = Arc::new(AtomicBool::new(false));
    let b = Arc::new(AtomicBool::new(false));
    let a_clone = a.clone();
    let b_clone = b.clone();

    let t1 = thread::spawn(move || {
        a.store(true, Ordering::Relaxed);
        if !b.load(Ordering::Relaxed) {
            S.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        b_clone.store(true, Ordering::Relaxed);
        if !a_clone.load(Ordering::Relaxed) {
            S.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn main() {
    let cnt = 100000;
    for i in 0..cnt {
        relaxed(i);
    }
    // 结果可能大于100000
    let s = S.load(Ordering::SeqCst);
    println!("s: {}", s);
}
// s: 99999
// s: 99996