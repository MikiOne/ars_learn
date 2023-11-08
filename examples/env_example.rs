use std::env;

/// 在idea环境变量中设置:
///
/// FOO=Egal nihao
fn main() {
    match env::var("FOO") {
        Ok(val) => println!("FOO: {}", val),
        Err(e) => println!("Couldn't interpret FOO: {}", e),
    }
}