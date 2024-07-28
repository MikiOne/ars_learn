// use snafu::{prelude::*, ResultExt};
// use std::fmt;
// use crate::DatabaseError::{ConnectionError, QueryError};
//
// // 定义自定义错误类型
// #[derive(Debug, Snafu)]
// enum DatabaseError {
//     #[snafu(display("Connection error: {}", message))]
//     ConnectionError { message: String },
//
//     #[snafu(display("Query error: {}", message))]
//     QueryError { message: String },
//
//     #[snafu(display("Data not found for user ID: {}", user_id))]
//     NotFound { user_id: u32 },
// }
//
// // 模拟数据库连接结构
// struct DatabaseConnection {
//     is_connected: bool,
// }
//
// impl DatabaseConnection {
//     fn new() -> Self {
//         DatabaseConnection { is_connected: false }
//     }
//
//     fn connect(&mut self) -> Result<(), DatabaseError> {
//         // 模拟连接操作
//         if rand::random() {
//             self.is_connected = true;
//             Ok(())
//         } else {
//             ConnectionError {
//                 message: "Failed to connect to database".to_string()
//             }.fail()
//         }
//     }
//
//     fn query(&self, query: &str) -> Result<Vec<String>, DatabaseError> {
//         if !self.is_connected {
//             return ConnectionError {
//                 message: "Not connected to database".to_string()
//             }.fail();
//         }
//
//         // 模拟查询操作
//         match query {
//             "SELECT * FROM users" => Ok(vec!["User1".to_string(), "User2".to_string()]),
//             "SELECT * FROM empty_table" => Ok(vec![]),
//             _ => QueryError {
//                 message: format!("Invalid query: {}", query)
//             }.fail(),
//         }
//     }
// }
//
// fn get_user_data(db: &DatabaseConnection, user_id: u32) -> Result<String, DatabaseError> {
//     let query = format!("SELECT * FROM users WHERE id = {}", user_id);
//     let results = db.query(&query).context(QuerySnafu { query })?;
//
//     results.get(0)
//         .cloned()
//         .ok_or_else(|| NotFound { user_id }.build())
// }
//
// fn main() -> Result<(), DatabaseError> {
//     let mut db = DatabaseConnection::new();
//
//     // 尝试连接数据库
//     db.connect().context(ConnectionSnafu)?;
//
//     // 尝试获取用户数据
//     match get_user_data(&db, 1) {
//         Ok(user) => println!("Found user: {}", user),
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             // 打印详细的错误信息和堆栈跟踪
//             eprintln!("{:?}", e);
//         }
//     }
//
//     Ok(())
// }
