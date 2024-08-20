fn main() {
    // Owned pointer – only one thing can ‘own’ this pointer at a time
    // This means that when the `Box` leaves its scope, it will be automatically deallocated safely.
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // dereference
    // Here, `now_its_mine` takes ownership of `mine`. In other words, `mine` is moved.
    let mut now_its_mine = mine;
    *now_its_mine += 2;

    println!("{}", now_its_mine); // 7
    // println!("{}", mine); // this would not compile because `now_its_mine` now owns the pointer
}
// error[E0382]: borrow of moved value: `mine`
//   --> week-one/a-base/examples/a4_owned_pointer.rs:11:20
//    |
// 4  |     let mut mine: Box<i32> = Box::new(3);
//    |         -------- move occurs because `mine` has type `Box<i32>`, which does not implement the `Copy` trait
// ...
// 7  |     let mut now_its_mine = mine;
//    |                            ---- value moved here
// ...
// 11 |     println!("{}", mine); // this would not compile because `now_its_mine` now owns the pointer
//    |                    ^^^^ value borrowed here after move
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 7  |     let mut now_its_mine = mine.clone();
//    |                                ++++++++