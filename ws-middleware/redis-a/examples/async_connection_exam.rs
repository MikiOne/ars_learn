use redis::AsyncCommands;


#[tokio::main]
async fn main() {
    get_key_as_bytes().await.unwrap();
}

async fn set_get() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://:eYVX7EwVmmxKPCDmwMtyKVge@127.0.0.1:6379/").unwrap();
    let mut con = client.get_async_connection().await?;

    con.set("key1", b"foo").await?;

    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut con)
        .await?;

    let result = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut con)
        .await;
    println!("result: {:?}", result);
    assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
    Ok(())
}

async fn get_key_as_bytes() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://:eYVX7EwVmmxKPCDmwMtyKVge@127.0.0.1:6379/").unwrap();
    let mut con = client.get_async_connection().await?;

    let result: Vec<u8> = redis::cmd("GET").arg("key1").query_async(&mut con).await?;
    println!("get_key_as_bytes: {:?}", result);  // 这应该会打印出你的字节数据
    Ok(())
}