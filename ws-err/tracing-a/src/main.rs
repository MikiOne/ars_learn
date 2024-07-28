use thiserror::Error;
use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::FmtSubscriber;

// 自定义错误类型
#[derive(Error, Debug)]
enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Data not found for user ID: {0}")]
    NotFound(u32),
}

// 模拟数据库连接结构
#[derive(Debug)]
struct DatabaseConnection {
    is_connected: bool,
}

impl DatabaseConnection {
    fn new() -> Self {
        DatabaseConnection { is_connected: false }
    }

    #[instrument]
    fn connect(&mut self) -> Result<(), DatabaseError> {
        debug!("Attempting to connect to database");
        // 模拟连接操作
        if rand::random() {
            self.is_connected = true;
            info!("Successfully connected to database");
            Ok(())
        } else {
            let err = DatabaseError::ConnectionError("Failed to connect to database".to_string());
            error!(?err, "Database connection failed");
            Err(err)
        }
    }

    #[instrument(skip(self))]
    fn query(&self, query: &str) -> Result<Vec<String>, DatabaseError> {
        if !self.is_connected {
            let err = DatabaseError::ConnectionError("Not connected to database".to_string());
            error!(?err, "Attempted query on disconnected database");
            return Err(err);
        }

        debug!(query = %query, "Executing database query");
        // 模拟查询操作
        match query {
            "SELECT * FROM users" => {
                let results = vec!["User1".to_string(), "User2".to_string()];
                info!(num_results = results.len(), "Query executed successfully");
                Ok(results)
            }
            "SELECT * FROM empty_table" => {
                warn!("Query returned no results");
                Ok(vec![])
            }
            _ => {
                let err = DatabaseError::QueryError(format!("Invalid query: {}", query));
                error!(?err, "Query execution failed");
                Err(err)
            }
        }
    }
}

#[instrument]
fn get_user_data(db: &DatabaseConnection, user_id: u32) -> Result<String, DatabaseError> {
    let query = format!("SELECT * FROM users WHERE id = {}", user_id);
    let results = db.query(&query)?;

    results.get(0)
        .cloned()
        .ok_or_else(|| {
            let err = DatabaseError::NotFound(user_id);
            error!(?err, "User data not found");
            err
        })
}

fn main() {
    // 设置 tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true).with_file(true)
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut db = DatabaseConnection::new();

    info!("Starting database operations");

    // 尝试连接数据库
    if let Err(err) = db.connect() {
        error!(?err, "Failed to establish database connection");
        return;
    }

    // 尝试获取用户数据
    match get_user_data(&db, 1) {
        Ok(user) => info!(user = %user, "Found user data"),
        Err(err) => error!(?err, "Failed to retrieve user data"),
    }

    info!("Database operations completed");
}
