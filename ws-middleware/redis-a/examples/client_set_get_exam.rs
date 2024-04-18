use redis::Commands;

fn fetch_an_integer() -> String {
    // cluster方式
    // let nodes = vec!["redis://:eYVX7EwVmmxKPCDmwMtyKVge@127.0.0.1:6379/"];
    // let client = ClusterClient::new(nodes).unwrap();

    // client方式二
    let client = redis::Client::open("redis://:eYVX7EwVmmxKPCDmwMtyKVge@127.0.0.1:6379/").unwrap();

    let mut connection = client.get_connection().unwrap();
    let _: () = connection.set("test", "test_data").unwrap();
    let rv: String = connection.get("test").unwrap();
    return rv;
}

fn main() {
    let res = fetch_an_integer();
    println!("fetch_an_integer result: {}", res);
}