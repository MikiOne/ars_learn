use clap::Parser;

#[derive(Parser)]
struct Cli {
    // 注意下面的注释是三个斜杠!!!
    /// argument of name
    name: String,
    /// argument of names
    names: Vec<String>,
}

/// cargo run -- zim name1 name2
fn main() {
    let cli = Cli::parse();
    let name = cli.name;
    let names = cli.names;
    println!("name: {name}");
    println!("names: {names:?}");
}