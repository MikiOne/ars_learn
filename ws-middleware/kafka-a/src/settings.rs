use std::env;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Eoplog {
    path: String,
    ser_name: String,
}

impl Eoplog {
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_ser_name(&self) -> &String {
        &self.ser_name
    }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct KafkaConfig {
    broker: String,
    topic: String,
}

impl KafkaConfig {
    pub fn get_broker(&self) -> &String { &self.broker }
    pub fn get_topic(&self) -> &String { &self.topic }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    pub eoplogs: Vec<Eoplog>,
    pub kafka_config: KafkaConfig
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let config_path = env::var("CONFIG_PATH").expect("请指定配置文件路径");
        let path = format!("{}-{}", config_path, run_mode);

        let config = File::with_name(&path).required(false);
        let setting = Config::builder().add_source(config).build()?;

        setting.try_deserialize()
    }

    // pub fn get_eoplogs(self) -> Vec<Eoplog> {
    //     self.eoplogs
    // }
    //
    // pub fn get_kafka_config(self) -> KafkaConfig {
    //     self.kafka_config
    // }

    pub fn is_debug(&self) -> bool {
        self.debug
    }
}
