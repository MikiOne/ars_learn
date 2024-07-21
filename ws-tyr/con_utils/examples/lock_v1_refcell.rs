use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::{fmt, thread};

struct Lock<T> {
    data: RefCell<T>,
    locked: RefCell<bool>,
}

unsafe impl<T> Sync for Lock<T> {}
impl<T> Debug for Lock<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
            locked: RefCell::new(false),
        }
    }

    fn lock(&self, op: impl FnOnce(&mut T)) {
        // 没有获取的锁，则spin
        while *self.locked.borrow() != false {}

        // 获取到锁则加锁
        *self.locked.borrow_mut() = true;
        // 如上报错：
        // thread '<unnamed>' panicked at ws-tyr/con_utils/examples/lock_exam.rs:34:22:
        // already borrowed: BorrowMutError

        // 执行
        op(&mut self.data.borrow_mut());

        // 释放锁
        *self.locked.borrow_mut() = false;
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|v| *v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v| *v *= 10);
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}
/*

thread '<unnamed>' panicked at ws-tyr/con_utils/examples/lock_exam.rs:34:22:
already borrowed: BorrowMutError
stack backtrace:
   0: rust_begin_unwind
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
   1: core::panicking::panic_fmt
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
   2: core::cell::panic_already_borrowed
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/cell.rs:770:5
   3: core::cell::RefCell<T>::borrow_mut
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/cell.rs:1061:25
   4: lock_exam::Lock<T>::lock
             at ./ws-tyr/con_utils/examples/lock_exam.rs:34:10
   5: lock_exam::main::{{closure}}
             at ./ws-tyr/con_utils/examples/lock_exam.rs:49:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at ws-tyr/con_utils/examples/lock_exam.rs:56:15:
called `Result::unwrap()` on an `Err` value: Any { .. }
stack backtrace:
   0: rust_begin_unwind
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
   1: core::panicking::panic_fmt
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
   2: core::result::unwrap_failed
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1654:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1077:23
   4: lock_exam::main
             at ./ws-tyr/con_utils/examples/lock_exam.rs:56:5
   5: core::ops::function::FnOnce::call_once
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
 */