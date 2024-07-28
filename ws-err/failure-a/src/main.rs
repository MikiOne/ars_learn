use failure::{Error, Fail};
use std::fmt;

// 自定义数据库错误类型
#[derive(Debug, Fail)]
enum DatabaseError {
    #[fail(display = "Connection error: {}", message)]
    ConnectionError { message: String },

    #[fail(display = "Query error: {}", message)]
    QueryError { message: String },

    #[fail(display = "Data not found")]
    NotFound,
}

// 模拟数据库连接结构
struct DatabaseConnection {
    is_connected: bool,
}

impl DatabaseConnection {
    fn new() -> Self {
        DatabaseConnection { is_connected: false }
    }

    fn connect(&mut self) -> Result<(), Error> {
        // 模拟连接操作
        if rand::random() {
            self.is_connected = true;
            Ok(())
        } else {
            Err(DatabaseError::ConnectionError {
                message: "Failed to connect to database".to_string(),
            }.into())
        }
    }

    fn query(&self, query: &str) -> Result<Vec<String>, Error> {
        if !self.is_connected {
            return Err(DatabaseError::ConnectionError {
                message: "Not connected to database".to_string(),
            }.into());
        }

        // 模拟查询操作
        match query {
            "SELECT * FROM users" => Ok(vec!["User1".to_string(), "User2".to_string()]),
            "SELECT * FROM empty_table" => Ok(vec![]),
            _ => Err(DatabaseError::QueryError {
                message: format!("Invalid query: {}", query),
            }.into()),
        }
    }
}

fn get_user_data(db: &DatabaseConnection, user_id: u32) -> Result<String, Error> {
    let query = format!("SELECT * FROM users WHERE id = {}", user_id);
    let results = db.query(&query)?;

    results.get(0)
        .cloned()
        .ok_or_else(|| DatabaseError::NotFound.into())
}

fn main() -> Result<(), Error> {
    let mut db = DatabaseConnection::new();

    // 尝试连接数据库
    db.connect()?; //.context("Failed to establish database connection")?;

    // 尝试获取用户数据
    match get_user_data(&db, 1) {
        Ok(user) => println!("Found user: {}", user),
        Err(e) => {
            println!("Error: {}", e);
            println!("Backtrace:\n{}", e.backtrace());
        }
    }

    Ok(())
}
