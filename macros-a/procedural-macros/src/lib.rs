use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_dervie(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(CustomDebug)]
pub fn custom_debug(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Customize the debug output for your type here
                f.debug_struct(stringify!(#name))
                    .field("field1", &self.field1)
                    .field("field2", &self.field2)
                    .finish()
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // `args` 用于接收属性参数的输入流，
    println!("attr: {:?}", attr);
    // `item` 用于接收被处理的输入流。
    println!("item: {:?}", item);

    let attr_vec: Vec<TokenTree> = attr.clone().into_iter().collect();
    println!("attr_vec: {:?}", attr_vec);
    let item_vec: Vec<TokenTree> = item.clone().into_iter().collect();
    println!("item_vec: {:?}", item_vec);
    item
}

#[proc_macro_attribute]
pub fn my_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析属性宏
    let mut args = attr.into_iter();
    let value = args.next().unwrap();
    println!("value = {:#?}", value.span().source_text());

    // 替换掉我们自己的代码
    // 输出一条语句，输出的内容是属性定义的值
    let modified_item = format!("fn main() {{ print!(\"Hello, \"); println!({}); }}",value);
    println!("modified_item: {:#?}", modified_item);

    // 返回我们自己的代码
    modified_item.parse().unwrap()
}