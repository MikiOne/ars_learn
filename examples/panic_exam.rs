use std::panic;

fn panic_catch() {
    let result = panic::catch_unwind(|| {
        println!("执行前");
        panic!("发生 panic");
        println!("执行后");
    });

    match result {
        Ok(_) => println!("无 Panic"),
        Err(_) => println!("捕获到 panic")
    }
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 { Err("除数不能为零") } else { Ok(a / b) }
}

fn exe_divide() {
    match divide(10, 0) {
        Ok(val) => { println!("结果：{}", val) }
        Err(err) => { println!("错误：{}", err); }
    }
}

#[test]
#[should_panic(expected = "除数不能为零")]
fn test_divide_by_zero() {
    divide(10, 0);
}

fn main() {
    exe_divide();
}
