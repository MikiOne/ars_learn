#[derive(Debug)]
struct Test {
    data: String,
    rData: *const String, // rData 是一个裸指针，引用了 data 的值
}

fn main() {
    let str = String::from("数据");
    let mut test = Test {
        data: str, // 1. 这里 str 的所有权转移到了这里，所以 rData 赋值为 &str 是会抛出所有权错误
        rData: std::ptr::null(), // 2. 所以这里，我们先初始化一个空指针
    };

    // 3. 获取 test.data 的引用然后再设置给 rData
    test.rData = &test.data; // 4. 此时 rData 的值为 0xc3560ff8c0
    println!("{:?}", test);

    // 现在的问题来了！！！！！！！！！！！
    // 如果我把 data 的值改变了呢？！！！！
    // 那 rData 会怎么样？
    test.data = String::from("我是修改后的数据");
    println!("{:?}", test.rData); // 5. 此时 rData 的值还是 0xc3560ff8c0 ！！！！！没有变！

    // 那这样的数据还是正确的数据吗？答案是否定的！

    // 那如何解决这个问题呢？有两种方法：
    // 1. 同时修改 data 和 rData 的值，可是你真的相信程序员吗？Rust 会相信我们不会出错吗？！
    //    所以即使我们保证不会出错，Rust 也不会允许这样的隐患存在，因为 Rust 自称是安全的语言！！！

    // 2. 因为 Rust 不信任我们，所以 Pin 诞生了，我把 test 固定住，不给你随便改，你要改必须按照我的规则来
    //    而且我还强制性让你加 unsafe 关键字，这样有锅也不是我的锅了，你（程序员）向我（Rust）保证了，这个代码我负责

    // 好了，责任分清楚了，我们都去干活吧 😁
}
