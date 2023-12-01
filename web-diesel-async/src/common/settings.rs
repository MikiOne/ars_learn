use std::env;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let config_path = env::var("CONFIG_PATH").expect("请指定配置文件路径");
        let path = format!("{}{}", config_path, run_mode);

        let config = File::with_name(&path).required(false);
        let s = Config::builder().add_source(config).build()?;

        s.try_deserialize()
    }

    pub fn get_database_url(self) -> String {
        self.database.url
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }
}
