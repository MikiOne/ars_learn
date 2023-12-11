//!
//! 声明式宏（Declarative Macros）：类似于模式匹配的方式，用于生成重复的代码。
//!
//! 声明式宏使用 macro_rules! 关键字定义。
//!

macro_rules! say_hello {
    () => {
        println!("Hello macro!")
    };
}

macro_rules! create_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("Function {:?} is called", stringify!($fn_name))
        }
    };
}

create_fn!(foo);
create_fn!(bar);

fn main() {
    foo();
    bar();
}
