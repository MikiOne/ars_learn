- 我该如何在redis-rs中使用密码和端口来连接到Redis服务器？
```shell
let client = redis::Client::open("redis://:[your_password]@127.0.0.1/")?;
let con = client.get_connection()?;
```
格式：`redis://:password@IP:port/`
注意: redis://后有`冒号(:)`和 password 分割。
