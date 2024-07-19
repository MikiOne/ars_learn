use std::thread;
use std::time::Duration;

fn main() {
    let duration = Duration::from_millis(1000);
    let handle = thread::spawn(move || {
        thread::sleep(duration);
        // 返回一个 String 类型
        return "hello".to_string();
    });

    let thread = handle.thread();
    println!("{:?}", thread);

    // 父线程接收子线程运行结束的结果
    let result = handle.join().unwrap();
    println!("{}", result);
}