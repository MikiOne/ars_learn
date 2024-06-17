use std::collections::HashMap;
use std::str::FromStr;
use anyhow::anyhow;
use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_derive::Args;
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, header, Response, Url};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;


/// ### run
///
/// cargo run -- get https://httpbin.org/get
///
/// cargo run -- post https://httpbin.org/post greeting=Hello
///
/// cargo run -- jsonp https://mpadminpro.hxdao.cn/mpa/login '{"username": "test", "password": "123456", "code": "0", "uuid": "e6259305aee84347a7644b560ee7a3b9"}'
/// ### help
/// cargo run -- --help
/// ### sub cmd help
/// cargo run -- post --help
///
/// cargo run -- get --help
///
/// ### post with header:
///
/// cargo run -- post https://httpbin.org/post greeting=Hello -d "Authorization=bearer token" -d "Authorization2=bearer token2"
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.cmd {
        Commands::Get(ref get) => get.do_get(client).await?,
        Commands::Post(ref post) => post.do_post(client).await?,
        Commands::Jsonp(ref post_json) => post_json.do_post(client).await?
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get request
    Get(Get),
    /// Post request
    Post(Post),
    /// Post request with json
    Jsonp(PostJson),
}

#[derive(Args, Debug)]
struct Get {
    /// 请求的网址url
    #[arg(value_parser = parse_url)]
    url: String,
}

fn parse_url(url: &str) -> Result<String> {
    let _: Url = url.parse()?;
    Ok(url.into())
}

impl Get {
    async fn do_get(&self, client: Client) -> Result<()> {
        let resp = client.get(&self.url).send().await?;
        Ok(print_resp(resp).await?)
    }
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut kv = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}, your input must be `k=v`", s));

        Ok(Self {
            k: kv.next().ok_or_else(err)?.to_string(),
            v: kv.next().ok_or_else(err)?.to_string(),
        })
    }
}

#[derive(Debug)]
struct HeaderMapWrapper(HeaderMap);

impl TryFrom<&Vec<KvPair>> for HeaderMapWrapper {
    type Error = anyhow::Error;
    fn try_from(value: &Vec<KvPair>) -> Result<Self, Self::Error> {
        let mut headers = HeaderMap::new();
        for kv in value.iter() {
            let key = HeaderName::from_str(kv.k.as_str())?;
            let val = HeaderValue::from_str(kv.v.as_str())?;
            headers.insert(key, val);
        }
        Ok(HeaderMapWrapper(headers))
    }
}

#[derive(Args, Debug)]
struct Post {
    /// 请求的网址url
    // #[arg(short('u'), long)]
    #[arg(value_parser = parse_url)]
    url: String,
    /// "name=zimu"这样格式的参数, 会转换为Post JSON body的类型
    // #[arg(short('b'), long)]
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
    /// 请求头，如”Authorization=bearer token“
    #[arg(short('d'), long)]
    #[arg(value_parser = parse_kv_pair)]
    header: Vec<KvPair>,
}

fn parse_kv_pair(body: &str) -> Result<KvPair> {
    Ok(body.parse()?)
}

impl Post {
    async fn do_post(&self, client: Client) -> Result<()> {
        println!("post header vec: {:?}", &self.header);
        let hmw: HeaderMapWrapper = (&self.header).try_into()?;
        println!("Request headers: {:?}", &hmw);

        let mut body = HashMap::new();
        for kv in self.body.iter() {
            body.insert(&kv.k, &kv.v);
        }

        let resp = client.post(&self.url).headers(hmw.0).json(&body).send().await?;
        Ok(print_resp(resp).await?)
    }
}

#[derive(Args, Debug)]
struct PostJson {
    /// 请求的网址url
    // #[arg(short('u'), long)]
    #[arg(value_parser = parse_url)]
    url: String,
    /// "name=zimu"这样格式的参数, 会转换为Post JSON body的类型
    // #[arg(short('b'), long)]
    #[arg(value_parser = parse_json)]
    json: Value,
    /// 请求头，如”Authorization=bearer token“
    #[arg(short('d'), long)]
    #[arg(value_parser = parse_kv_pair)]
    header: Vec<KvPair>,
}

fn parse_json(s: &str) -> Result<Value> {
    let val: Value = serde_json::from_str(s)?;
    Ok(val)
}

impl PostJson {
    async fn do_post(&self, client: Client) -> Result<()> {
        let hmw: HeaderMapWrapper = (&self.header).try_into()?;
        let resp = client.post(&self.url).headers(hmw.0).json(&self.json).send().await?;
        Ok(print_resp(resp).await?)
    }
}


// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    print!("\n");
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_json;

    #[test]
    fn test_parse_json() -> anyhow::Result<()> {
        let json_str = r#"{"username": "test", "password": "123456", "code": "0", "uuid": "e6259305aee84347a7644b560ee7a3b9"}"#;
        let json = parse_json(json_str)?;
        Ok(println!("{json:?}"))
    }
}