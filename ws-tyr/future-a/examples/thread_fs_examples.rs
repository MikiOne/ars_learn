use std::{fs, thread};
use std::thread::JoinHandle;

use anyhow::{anyhow, Result};
use serde_yaml::Value;

struct MyJoinHandle<T>(JoinHandle<Result<T>>);
impl<T> MyJoinHandle<T> {
    /// 等待 thread 执行完（类似 await）
    fn thread_await(self) -> Result<T> {
        self.0.join().map_err(|_| anyhow!("thread await failed"))?
    }
}


fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let t1 = read_via_thread("./Cargo.toml");
    // 读取 Cargo.lock，IO 操作 2
    let t2 = read_via_thread("./Cargo.lock");

    let content1 = t1.thread_await()?;
    let content2 = t2.thread_await()?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let t3 = write_via_thread("/tmp/Cargo.yml", yaml1);
    let t4 = write_via_thread("/tmp/Cargo.lock", yaml2);

    let yaml1 = t3.thread_await()?;
    let yaml2 = t4.thread_await()?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}
fn read_via_thread(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let content = fs::read_to_string(filename)?;
        // Ok::<_, anyhow::Error>(content)
        Ok(content)
    });
    MyJoinHandle(handle)
}
fn write_via_thread(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content)?;
        // Ok::<_, anyhow::Error>(content)
        Ok(content)
    });
    MyJoinHandle(handle)
}