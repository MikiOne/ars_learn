use std::rc::Rc;

/// 文档：https://mp.weixin.qq.com/s/zr7_9xbWYgYMAOVBda4Dxg

/// 所有权：
///
/// Rust中所有权约束了值只能有一个所有者，当值离开作用域时，它将被销毁。
///
/// 像如下代码，字符串a如果直接移动给b后就没法后边再去打印，因为它的所有权已经转移给了b。
fn own() {
    // let a = String::from("Helll");
    // let b = a;
    // println!("a = {}, b = {}", a, b);

    // 编译结果
    //   |
    // 5 |     let a = String::from("Helll");
    //   |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
    // 6 |     let b = a;
    //   |             - value moved here
    // 7 |     println!("a = {}, b = {}", a, b);
    //   |                                ^ value borrowed here after move
    //   |
    //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // help: consider cloning the value if the performance cost is acceptable
    //   |
    // 6 |     let b = a.clone();
    //   |              ++++++++
}

/// 深拷贝（clone）
///
///如果clone的话可以复制一份，但是这样的话就需要开辟一块新的内存，不是很高效。
fn test_clone() {
    let a = String::from("Helll");
    let b = a.clone();
    println!("a = {}, b = {}", a, b);
}

/// 引用计数（reference count）
///
///想要实现多个所有者，又开销小，可以用引用计数，对应的类型是Rc。
///
/// Rc只会在复制时增加引用计数，当引用计数为 0 时，会自动调用drop方法，释放内存。
fn test_rc() {
    let a = Rc::new(String::from("Helll"));
    let b = Rc::clone(&a);
    println!("reference count {}", Rc::strong_count(&a));

    {
        let b = Rc::clone(&a);
        println!("reference count {}", Rc::strong_count(&a));
        // 3, will be 2 after this block out of scope
    }

    println!("reference count {}", Rc::strong_count(&a));
}

/// Rc引用的值是不可变的，如果想要修改，可以使用Rc::make_mut方法，它会检查引用计数，在有别的有效引用（strong）时，会复制一份，然后修改。否则就直接修改原来的值。这也是写时复制，只有在需要修改时才会复制。
///
/// 所以这么用有一个好处，如果有修改，修改是独立于之前的引用的，不用担心修改会影响之前引用的值。
///
/// 当然，如果想保持值修改的同步，可以使用之前提到的Cell和RefCell，这两个类型可以实现内部可变性，可以在不可变引用的情况下修改值。
fn copy_on_write() {
    let mut a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    (*Rc::make_mut(&mut a)).push_str(" world");
    println!("a: {}, b: {}", a, b); // a: hello world, b: hello

    let c = Rc::clone(&a);
    println!("a: {}, b: {}, c: {}", a, b, c); // a: hello world, b: hello, c: hello world
}
/// 循环引用
///
/// Rc是不允许循环引用的，因为它的引用计数是在编译时就确定的，如果有循环引用，那么引用计数永远不会为 0，也就永远不会调用drop方法，导致内存泄漏。
mod cycle_rc {
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Rc<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }
    pub fn test_cycle_rc() {
        let gadget_owner: Rc<Owner> =
            Rc::new(Owner { name: String::from("Gadget Man"), gadgets: RefCell::new(vec![]) });

        //两个工具，都有同一个所有者
        let gadget1 = Rc::new(Gadget { id: 1, owner: gadget_owner.clone() });
        let gadget2 = Rc::new(Gadget { id: 2, owner: gadget_owner.clone() });

        gadget_owner.gadgets.borrow_mut().push(gadget1.clone());
        gadget_owner.gadgets.borrow_mut().push(gadget2.clone());

        // 释放gadget_owner的引用计数，保留工具的owner引用计数
        drop(gadget_owner);

        println!("strong count of gadget1: {}", Rc::strong_count(&gadget1));
        // strong count of gadget1: 2

        println!("strong count of gadget1.owner: {}", Rc::strong_count(&gadget1.owner));
        // strong count of gadget1.owner: 2

        // 释放gadget1的引用计数，正常没有引用循环的话，owner对应的引用计数也需要释放
        // 但是gadget1的owner的引用计数不会减一，导致内存泄漏
        drop(gadget1);

        println!("strong count of gadget2.owner: {}", Rc::strong_count(&gadget2.owner));
        // strong count of gadget2.owner: 2
    }
}

///循环引用如下图所示
///
/// gadgets和owner的引用形成了一个环，谁也没法释放，对应的引用计数无法减到0，也就没法释放
///
/// +-----------+       +-----------+
/// |   Owner   |<------|  Gadget   |
/// |           |       |           |
/// |   Rc      |       |   Rc      |
/// |           |       |           |
/// | gadgets --|------>| owner ----+
/// +-----------+       +-----------+
///

///弱引用
///
/// 这个时候就是弱引用的用武之地了，弱引用不会增加引用计数，所以不会导致循环引用。
///
/// 但是它也不能保证引用的值一定存在，因为它的引用计数可能为 0，所以用时，需要用upgrade方法来获取Option类型的引用。
///
/// 也就是说引用的值释放与否只取决于强引用的引用计数。
mod weak {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::rc::Weak;

    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    pub fn test_weak() {
        let gadget_owner: Rc<Owner> =
            Rc::new(Owner { name: "Gadget Man".to_string(), gadgets: RefCell::new(Vec::new()) });

        let gadget1 = Rc::new(Gadget { id: 1, owner: gadget_owner.clone() });
        let gadget2 = Rc::new(Gadget { id: 2, owner: gadget_owner.clone() });

        gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1.clone()));
        gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2.clone()));

        for gadget_opt in gadget_owner.gadgets.borrow().iter() {
            let gadget = gadget_opt.upgrade().unwrap();
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }
        drop(gadget_owner);
        println!("strong count of gadget1: {}", Rc::strong_count(&gadget1));
        // strong count of gadget1: 1
        println!("strong count of gadget1.owner: {}", Rc::strong_count(&gadget1.owner));
        // strong count of gadget1.owner: 2
        drop(gadget1);

        println!("strong count of gadget2.owner: {}", Rc::strong_count(&gadget2.owner));
        // strong count of gadget2.owner: 1
    }
}

/// 线程安全
// Rc是线程不安全的，如果想要在多线程中使用，可以使用Arc，它是Rc的线程安全版本。（A代表atomic）

// 而如果想要在多线程中修改值，可以使用Mutex和RwLock，它们都是线程安全的。如Arc<Mutex<T>>。
//
// 最后还有一点想提下，Rc<T>和Arc<T>都实现了自动解引用Deref到T，所以可以直接在Rc<T>和Arc<T>上调用T的方法。而为了防止方法名冲突，一般习惯用全限定语法调用方法来调用Rc<T>和Arc<T>的方法，如Rc::clone。
mod thread_safe {
    use std::sync::Arc;
    use std::thread;

    pub fn test() {
        let val = Arc::new(5);

        for _ in 0..3 {
            let val = Arc::clone(&val);
            thread::spawn(move || {
                let v = *val.as_ref() + 1;
                println!("{v:?}");
            });
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    thread_safe::test();
}
