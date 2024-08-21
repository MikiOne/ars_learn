// rscli csv -i input.csv -o output.json --header -d ','

use std::fs::File;
use std::path::Path;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rscli", version, about, long_about = None)]
struct RsCli {
    #[command(subcommand)]
    cmd: SubCommand,
}
#[derive(Subcommand, Debug)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts)
}

#[derive(Parser, Debug)]
struct CsvOpts {
    /// input CSV filename
    #[arg(short, long, value_parser = check_file_exists)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(long, default_value_t = true)]
    header: bool,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn check_file_exists(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("Cannot open file {:?}", filename))
    }
    // if let Ok(_file) = File::open(filename) {
    //     Ok(filename.into())
    // } else {
    //     Err(format!("Cannot open file {:?}", filename))
    // }
}

// rscli csv -i input.csv -o output.json --header -d ','
fn main() {
    let cli = RsCli::parse();

    println!("rscli: {:?}", cli);
}

