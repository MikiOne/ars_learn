use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread,
};

static S: AtomicU64 = AtomicU64::new(0);

// 死循环
fn relaxed() {
    let cnt = 100000;

    let a = Arc::new(AtomicBool::new(false));
    let b = Arc::new(AtomicBool::new(false));
    let a_clone = a.clone();
    let b_clone = b.clone();

    let mut handlers = Vec::new();

    for _ in 0..cnt {
        let a = a.clone();
        let b = b.clone();
        let t1 = thread::spawn(move || {
            a.store(true, Ordering::Release);
            if !b.load(Ordering::Acquire) {
                S.fetch_add(1, Ordering::Release);
            }
        });
        handlers.push(t1);
    }


    for _ in 0..cnt {
        let a_clone = a_clone.clone();
        let b_clone = b_clone.clone();
        let t2 = thread::spawn(move || {
            b_clone.store(true, Ordering::Release);
            if !a_clone.load(Ordering::Acquire) {
                S.fetch_add(1, Ordering::Release);
            }
        });
        handlers.push(t2);
    }

    for t in handlers {
        t.join().unwrap();
    }
}

fn main() {
    relaxed();

    // 结果可能大于10000
    let s = S.load(Ordering::SeqCst);
    println!("s: {}", s);
}
/*
thread 'mainthread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:182:13:
failed to set up alternative stack guard page: Cannot allocate memory (os error 12)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
stack backtrace:
' panicked at thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
/rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/thread/mod.rs:697:29:
failed to spawn thread: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:thread '<unnamed>thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
178thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)thread 'thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178::thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:1313:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
<unnamed>' panicked at thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)

:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)

thread 'thread '<unnamed>' panicked at thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:182:13:
failed to set up alternative stack guard page: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread 'thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:thread '<unnamed>' panicked at thread 'thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread 'thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread 'thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
<unnamed>178library/std/src/sys/pal/unix/stack_overflow.rs:' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:thread '<unnamed>thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at thread '<unnamed>' panicked at library/std/src/sys/pal/unix/stack_overflow.rs' panicked at library/std/src/sys/pal/unix/stack_overflow.rslibrary/std/src/sys/pal/unix/stack_overflow.rs:178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)178:178' panicked at :178178::library/std/src/sys/pal/unix/stack_overflow.rs:178::13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)

178:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
:13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
13:
failed to allocate an alternative stack: Cannot allocate memory (os error 12)
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread 'thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread 'thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread 'thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread 'thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
<unnamed>' panicked at library/core/src/panicking.rs:thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
220:5:
panic in a function that cannot unwind
:5:
panic in a function that cannot unwind
thread 'thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
panic in a function that cannot unwind
<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
panic in a function that cannot unwind
:5:
panic in a function that cannot unwind
:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rsthread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
:220:5:
panic in a function that cannot unwind
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
:5thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
:
panic in a function that cannot unwind
thread 'thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220thread ':5<unnamed>:
' panicked at panic in a function that cannot unwindlibrary/core/src/panicking.rs
:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
<unnamed>' panicked at library/core/src/panicking.rs:220:5:
panic in a function that cannot unwind
   0:     0x564f49832545 - std::backtrace_rs::backtrace::libunwind::trace::h1a07e5dba0da0cd2
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/libunwind.rs:105:5
   1:     0x564f49832545 - std::backtrace_rs::backtrace::trace_unsynchronized::h61b9b8394328c0bc
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x564f49832545 - std::sys_common::backtrace::_print_fmt::h1c5e18b460934cff
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:68:5
   3:     0x564f49832545 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1e1a1972118942ad
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x564f4984fbbb - core::fmt::rt::Argument::fmt::h07af2b4071d536cd
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/rt.rs:165:63
   5:     0x564f4984fbbb - core::fmt::write::hc090a2ffd6b28c4a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/mod.rs:1157:21
   6:     0x564f4983077f - std::io::Write::write_fmt::h8898bac6ff039a23
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/io/mod.rs:1832:15
   7:     0x564f4983231e - std::sys_common::backtrace::_print::h4e80c5803d4ee35b
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x564f4983231e - std::sys_common::backtrace::print::ha96650907276675e
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x564f498335d9 - std::panicking::default_hook::{{closure}}::h215c2a0a8346e0e0
  10:     0x564f4983331d - std::panicking::default_hook::h207342be97478370
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:298:9
  11:     0x564f49833a73 - std::panicking::rust_panic_with_hook::hac8bdceee1e4fe2c
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:795:13
  12:     0x564f4983391b - std::panicking::begin_panic_handler::{{closure}}::h00d785e82757ce3c
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:656:13
  13:     0x564f49832a09 - std::sys_common::backtrace::__rust_end_short_backtrace::h1628d957bcd06996
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:171:18
  14:     0x564f49833687 - rust_begin_unwind
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
  15:     0x564f4980c070 - core::panicking::panic_nounwind_fmt::runtime::habe92a792a272e00
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:110:18
  16:     0x564f4980c070 - core::panicking::panic_nounwind_fmt::h9763d8b18bc7b832
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:120:5
  17:     0x564f4980c102 - core::panicking::panic_nounwind::h23e6f792ad66b857
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:220:5
  18:     0x564f4980c1e6 - core::panicking::panic_cannot_unwind::h39dea8c15007a88a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:308:5
  19:     0x564f498359f4 - std::sys::pal::unix::thread::Thread::new::thread_start::h522bc89a54da820a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys/pal/unix/thread.rs:102:9
  20:     0x7fe6e9c1c609 - start_thread
  21:     0x7fe6e9b3b353 - clone
  22:                0x0 - <unknown>
stack backtrace:
   0:     0x564f49832545 - std::backtrace_rs::backtrace::libunwind::trace::h1a07e5dba0da0cd2
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/libunwind.rs:105:5
   1:     0x564f49832545 - std::backtrace_rs::backtrace::trace_unsynchronized::h61b9b8394328c0bc
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x564f49832545 - std::sys_common::backtrace::_print_fmt::h1c5e18b460934cff
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:68:5
   3:     0x564f49832545 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1e1a1972118942ad
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x564f4984fbbb - core::fmt::rt::Argument::fmt::h07af2b4071d536cd
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/rt.rs:165:63
   5:     0x564f4984fbbb - core::fmt::write::hc090a2ffd6b28c4a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/mod.rs:1157:21
   6:     0x564f4983077f - std::io::Write::write_fmt::h8898bac6ff039a23
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/io/mod.rs:1832:15
   7:     0x564f4983231e - std::sys_common::backtrace::_print::h4e80c5803d4ee35b
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x564f4983231e - std::sys_common::backtrace::print::ha96650907276675e
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x564f498335d9 - std::panicking::default_hook::{{closure}}::h215c2a0a8346e0e0
  10:     0x564f4983331d - std::panicking::default_hook::h207342be97478370
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:298:9
  11:     0x564f49833a73 - std::panicking::rust_panic_with_hook::hac8bdceee1e4fe2c
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:795:13
  12:     0x564f4983391b - std::panicking::begin_panic_handler::{{closure}}::h00d785e82757ce3c
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:656:13
  13:     0x564f49832a09 - std::sys_common::backtrace::__rust_end_short_backtrace::h1628d957bcd06996
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:171:18
  14:     0x564f49833687 - rust_begin_unwind
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
  15:     0x564f4980c070 - core::panicking::panic_nounwind_fmt::runtime::habe92a792a272e00
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:110:18
  16:     0x564f4980c070 - core::panicking::panic_nounwind_fmt::h9763d8b18bc7b832
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:120:5
  17:     0x564f4980c102 - core::panicking::panic_nounwind::h23e6f792ad66b857
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:220:5
  18:     0x564f4980c1e6 - core::panicking::panic_cannot_unwind::h39dea8c15007a88a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:308:5
  19:     0x564f498359f4 - std::sys::pal::unix::thread::Thread::new::thread_start::h522bc89a54da820a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys/pal/unix/thread.rs:102:9
  20:     0x7fe6e9c1c609 - start_thread
  21:     0x7fe6e9b3b353 - clone
  22:                0x0 - <unknown>
thread caused non-unwinding panic. aborting.
 */