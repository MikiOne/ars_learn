use std::collections::HashMap;
macro_rules! map {
    (
        $($key: expr => $value: expr),*
    ) => {
        {
            let mut hm = HashMap::new();
            // 应该在Rust宏的转码器中创建哈希表对象，
            // 然后将捕捉到的所有键值对插入该哈希表：
            $(hm.insert($key, $value);)*
            hm
        }
    };
}
fn main() {
    let mut user = map!(
        "name" => "Miki",
        "gender" => "男人"
    );
    println!("User: {:?}", user);
    user.insert("name", "egal");
    println!("User: {:?}", user);
}