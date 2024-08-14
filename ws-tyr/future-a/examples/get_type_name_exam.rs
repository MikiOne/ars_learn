fn main() {
    let fut = async { 42 };

    println!("type of fut is: {}", get_type_name(&fut));
    GenFuture
}

fn get_type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}