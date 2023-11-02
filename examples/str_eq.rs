fn eq_string() {
    let str1 = String::from("Hello");
    let str2 = String::from("Hello");

    if str1 == str2 {
        println!("The strings are equal.");
    } else {
        println!("The strings are not equal.");
    }
}

fn main() {
    let str1: &str = "Hello";
    let str2: &str = "Hello";

    if str1 == str2 {
        println!("The strings are equal.");
    } else {
        println!("The strings are not equal.");
    }
}
