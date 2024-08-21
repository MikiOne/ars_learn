// rscli csv -i input.csv -o output.json --header -d ','

use std::fs::File;
use clap::{Args, Parser, Subcommand};
use rscli::converter::convert;

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
    if let Ok(_file) = File::open(filename) {
        Ok(filename.to_string())
    } else {
        Err(format!("Cannot open file {:?}", filename))
    }
}

// rscli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let cli = RsCli::parse();
    println!("rscli: {:?}", cli);

    match cli.cmd {
        SubCommand::Csv(opts) => {
            println!("CsvOpts: {:?}", opts);
            convert(&opts.input, &opts.output)?
        }
    }

    Ok(())
}

