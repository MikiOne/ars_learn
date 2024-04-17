use clap::{Arg, Command};
use reqwest::Url;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置命令行参数
    let matches = Command::new("httpme")
        .version("1.0")
        .author("Your name <your.email@domain.com>")
        .about("Does awesome things")
        .subcommand(
            Command::new("post")
                .about("Performs an HTTP POST request")
                .arg(Arg::new("url").required(true).index(1))
                .arg(Arg::new("param").required(false).multiple_values(true))
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("post") {
        // URL 参数为必选，所以直接 unwrap
        let url = matches.value_of("url").unwrap();
        let url = Url::parse(url)?;

        // 参数是可选的，如果存在就解析为 HashMap
        let mut params = HashMap::new();
        if let Some(values) = matches.values_of("param") {
            for param_str in values {
                let parts: Vec<&str> = param_str.splitn(2, '=').collect();
                if parts.len() == 2 {
                    params.insert(parts[0], parts[1]);
                }
            }
        }

        // 发送 HTTP POST 请求
        let client = reqwest::Client::new();
        let response = client.post(url).json(&params).send().await?;
        println!("{:?}", response.text().await?);
    }

    Ok(())
}