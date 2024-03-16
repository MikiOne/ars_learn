fn fn_once() {
    let text = String::from("hello");
    let consume = move ||{
        println!("{}", text);
    };
    consume();
    // println!("{}", text); // 这会报错，因为text已经被消费
}

fn mut_fn() {
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    increment(); // 输出: Count: 1
    increment(); // 输出: Count: 2
}

fn immut_fn() {
    let text = String::from("hello");
    let print = || {
        println!("{}", text);
    };
    print(); // 输出: hello
    print(); // 输出: hello，可以再次调用
}

fn main() {
    // fn_once();

    // mut_fn();

    immut_fn();
}