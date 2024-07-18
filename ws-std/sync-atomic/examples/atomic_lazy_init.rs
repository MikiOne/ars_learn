use std::{
    sync::atomic::{AtomicPtr, Ordering},
    thread,
};

struct Data {}

fn generate_data() -> Data {
    Data {}
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Ordering::Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(),
            p,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            // Safety: p comes from Box::into_raw right above,
            // and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}

fn main() {
    let t1 = thread::spawn(|| get_data());
    let t2 = thread::spawn(|| get_data());
    let (ret1, ret2) = (t1.join().unwrap(), t2.join().unwrap());
    assert_eq!(ret1 as *const _, ret2 as *const _);
}
