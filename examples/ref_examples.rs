fn main() {
    // ref_box();
    ref_deref();
}

fn ref_deref() {
    let x = MyBox::new(10);
    println!("Value using our smart pointer: {}", *x); // prints correctly

    let x = Box::new(String::from("some string literal"));

    // Box -> String -> str -> &str
    take_str(&(*(*x)));

    // Box -> String 然后取一个字符串切片来获取&str
    take_str(&(*x)[..]);

    // 在Box上调用deref，然后在String上调用
    take_str(x.deref().deref());

    // 执行与上述等价的强制解引用
    take_str(&x);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn take_str(str: &str) {
    // does whatever
}

fn ref_box() {
    // 值在栈中
    let x = 10;
    // 指向x的常规指针
    let x_ptr = &x;
    // 解引用指针
    println!("Value using regular pointer: {}", *x_ptr);
    println!("Value using regular pointer: {}", x_ptr);

    // 指向堆上整数的智能指针(Box)
    let x = Box::new(10);
    // 自动的对潜在的值进行解引用
    println!("Value using smart pointer: {}", *x);
    println!("Value using smart pointer: {}", x);
}
