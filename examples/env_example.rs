use std::env;

fn main() {
    env_vars_iter();
}

/// evn::args
///
/// 在命令行command后添加：
/// -- DBG debug_test
/// 注意: 参数符号"--"与参数名之间有空格。
///
/// 如：run --package ars_learn --example env_example -- DBG debug_test
///
/// 运行：
///
/// [examples/env_example.rs:11] args = [
///     "target/debug/examples/env_example",
///     "DBG",
///     "debug_test",
/// ]
fn env_args_dbg() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

/// env::var
///
/// 在idea环境变量中设置:
///
/// FOO=Egal nihao
fn env_var_foo() {
    match env::var("FOO") {
        Ok(val) => println!("FOO: {}", val),
        Err(e) => println!("Couldn't interpret FOO: {}", e),
    }
}

/// 同样, env::vars()返回的也是一个迭代器, 不同的是, 数据结构是以key:value对的形式保存, 程序中分别将key和value进行打印
fn env_vars_iter() {
    for (key, value) in env::vars() {
        println!("{} => {}", key, value);
    }
}
