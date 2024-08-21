use std::fs::File;
use std::io;
use std::io::Write;
use csv::ReaderBuilder;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    #[serde(rename = "下线ID")]
    user_id: u64,
    #[serde(rename = "下线昵称")]
    nick_name: String,
    #[serde(rename = "下线等级")]
    below_level: String,
    #[serde(rename = "注册时间")]
    register_time: String,
    #[serde(rename = "累计充值金额")]
    deposit_amount: f32,
    #[serde(rename = "累计下注金额")]
    bet_amount: f32,
    #[serde(rename = "累计体现金额")]
    withdraw_amount: f32,
    #[serde(rename = "返佣贡献金额")]
    commission_amount: f32,
}

pub fn convert(input: &str, output: &str) -> anyhow::Result<()> {
    let mut records = Vec::with_capacity(256);
    let mut rdr = ReaderBuilder::new().from_path(input)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
        records.push(record);
    }

    let json_str = serde_json::to_string_pretty(&records)?;
    let mut file = File::create(&output)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}