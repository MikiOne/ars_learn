mod raw_builder;

use proc_macro::TokenStream;
use crate::raw_builder::BuilderContext;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() {println!(\"Hello, World!\");}".parse().unwrap()
}

// #[proc_macro_derive(RawBuilder)]
// pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
//     println!("{:#?}", input);
//     TokenStream::default()
// }

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    BuilderContext::render(input).unwrap().parse().unwrap()
}

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    TokenStream::default()
}