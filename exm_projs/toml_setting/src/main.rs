use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

// #[macro_use]
// extern crate lazy_static;

fn main() {
    // let ss = Setting::get();
    // println!("Setting result:\n {:#?}", ss);

    read_toml();
}

pub fn read_toml() {
    let setting = Setting::default();
    println!("{:#?}", setting);
    let c = Setting::get();
    println!("{:#?}", c);
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub debug: bool,
    pub debug_sql: bool,
    pub log_root: String,
}
#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub database: Database,
    pub log: Log,
}

impl Default for Setting {
    fn default() -> Self {
        let file_path =
            "/Users/egal/workspace/rust_ws/ars_learn/exm_projs/toml_setting/config.toml";

        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("error is op en config {e}"),
        };

        let mut str = String::new();
        println!("{}", str);
        match file.read_to_string(&mut str) {
            Ok(s) => s,
            Err(e) => panic!("error read str {}", e),
        };

        toml::from_str(&str).expect("Parsing the configuration file failed")
    }
}

impl Setting {
    pub fn get<'a>() -> &'a Self {
        lazy_static! {
            static ref CACHE: Setting = Setting::default();
        }
        &CACHE
    }
}
