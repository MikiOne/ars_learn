use std::collections::HashMap;
use std::mem::size_of_val;

fn main() {
    // 长度为：0
    let c1 = || println!("hello world!");

    // 和参数无关，长度也为 0
    let c2 = |i: i32| println!("hello {}!", i);

    let name = String::from("Egal");
    println!("name size: {}", size_of_val(&name));
    println!("&name size: {}", size_of_val(&(&name)));
    let name1 = name.clone();
    println!("name1 size: {}", size_of_val(&name1));

    let mut table = HashMap::new();
    println!("empty table size: {}", size_of_val(&table));
    table.insert("hello", "world");
    println!("table size: {}", size_of_val(&table));

    // 如果捕获一个引用，长度为 8
    let c3 = || println!("hello: {}", name);

    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);

    let name2 = name.clone();
    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let c5 = move ||{
        let x = 1;
        let name3 = String::from("Miki");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}