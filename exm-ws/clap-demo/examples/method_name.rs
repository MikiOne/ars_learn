use clap::Parser;

#[derive(Parser)]
#[command(name = "zimu")]
#[command(author = "zimu.top")]
#[command(version = "1.0")]
#[command(about = "a tutorial of crate clap", long_about = None)]
struct Cli {
    // 注意下面的注释是三个斜杠!!!
    /// use which method
    #[arg(short, long)]
    method: Option<String>,

    /// Optional name to call
    name: Option<String>,
}

/// cargo run -- -m zim name1
fn main() {
    let cli = Cli::parse();
    let method = cli.method.unwrap_or_else(|| "hello".to_owned());
    let name = cli.name.unwrap_or_else(|| "world".to_owned());
    println!("{method} {name}");
}