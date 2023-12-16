use procedural_macros::{CustomDebug, HelloMacro, my_main, test_macro};

// #[derive(HelloMacro)]
// struct Pancakes;



// #[my_main("Rust_main")]
// fn main() {
//     Pancakes::hello_macro();
//     // test_macro_test();
// }

/// 执行cargo build后，在控制台输出如下
///
/// 参数 attr 就是属性宏 #[test_macro(name = "zhangsan", age = 18)] 的分解：
/// attr: TokenStream [Ident { ident: "name", span: #0 bytes(158..162) }, Punct { ch: '=', spacing: Alone, span: #0 bytes(163..164) }, Literal { kind: Str, symbol: "MikiOng", suffix: None, span: #0 bytes(165..174) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(174..175) }, Ident { ident: "age", span: #0 bytes(176..179) }, Punct { ch: '=', spacing: Alone, span: #0 bytes(180..181) }, Literal { kind: Integer, symbol: "18", suffix: None, span: #0 bytes(182..184) }]
///
/// 参数item 对代码的分解：
/// item: TokenStream [Ident { ident: "fn", span: #0 bytes(187..189) }, Ident { ident: "test_macro_test", span: #0 bytes(190..205) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(205..207) }, Group { delimiter: Brace, stream: TokenStream [Ident { ident: "println", span: #0 bytes(214..221) }, Punct { ch: '!', spacing: Alone, span: #0 bytes(221..222) }, Group { delimiter: Parenthesis, stream: TokenStream [Literal { kind: Str, symbol: "Hello, World!", suffix: None, span: #0 bytes(223..238) }], span: #0 bytes(222..239) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(239..240) }], span: #0 bytes(208..242) }]
#[test_macro(name = "MikiOng", age = 18)]
fn test_macro_test() {
    println!("Hello, World!");
}

#[derive(CustomDebug)]
struct MyStruct {
    field1: i32,
    field2: String,
}

fn main() {
    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello, world!".into(),
    };
    println!("{:#?}", my_struct);
}