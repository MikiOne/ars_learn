fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

type FunctionPointer = fn(u32) -> u32;

fn main() {
    let fib: FunctionPointer = fibonacci;
    println!("Fib: {}", fib(4)); // 5
}