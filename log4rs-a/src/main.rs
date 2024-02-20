#[macro_use]
extern crate log;
extern crate log4rs;

use std::path::PathBuf;

use log4rs::append::rolling_file::{policy, RollingFileAppender};
use log4rs::config::{Appender, Root};
use log::info;
use log::LevelFilter;
use ntex::web::{App, HttpServer, middleware};
use rand::Rng;

use log4rs_a::{Env, env};

use crate::log4rs_config::ConfigLog4rs;

pub mod log4rs_config;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    Env::init_env(env());

    // let window_roller = FixedWindowRoller::builder().build("./logs/backup{}.gz", 30).unwrap();
    // // 滚动触发阈值设为15MB。
    // let size_trigger = SizeTrigger::new(1 * 1024 * 1024);
    // let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));
    //
    // let log_file = RollingFileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
    //     .build("logs/app.log", Box::new(compound_policy)).unwrap();
    //
    // let config = Config::builder()
    //     .appender(Appender::builder().build("log_file", Box::new(log_file)))
    //     .build(Root::builder().appender("log_file").build(LevelFilter::Info)).unwrap();
    //
    // log4rs::init_config(config).unwrap();
    //
    // info!("This is an information log message!");
    // error!("This is an error message!");
    // test_log_files();

    let log_path = PathBuf::from("./logs");
    ConfigLog4rs::new(&log_path).unwrap().init_config().unwrap();

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
    })
        .bind(&bind)?
        .run()
        .await
}


// fn fn_date_roller() {
//     let pattern = "{d(%Y-%m-%d %H:%M:%S)} - {l} - {m}\n";
//
//     let roller = log4rs::append::rolling_file::policy::compound::roll::date::DateRoller::new();
//     let trigger = SizeTrigger::new(10 * 1024 * 1024); // Size in bytes. Here 10 MB
//
//     let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
//
//     let appender = RollingFileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(pattern)))
//         .build("logs/app.log", Box::new(policy))
//         .unwrap();
//
//     let config = Config::builder()
//         .appender(Appender::builder().build("appender", Box::new(appender)))
//         .build(Root::builder().appender("appender").build(LevelFilter::Info))
//         .unwrap();
//
//     log4rs::init_config(config).unwrap();
//
//     info!("Hello, log file!");
// }

fn fn_policy() {
    // 创建长度触发器，当文件长度超过50MB时进行滚动
    let trigger = policy::compound::trigger::size::SizeTrigger::new(50 * 1024 * 1024);

    // 创建 Fixed Window 滚动器，这会在滚动时重命名旧的日志文件。这里我们设置它保存5个旧的日志文件。
    let roller = policy::compound::roll::fixed_window::FixedWindowRoller::builder().build("foo.log.{}", 5).unwrap();

    // 创建 Compound 政策，它可以包含多个滚动 触发器 和 滚动器
    let policy = policy::compound::CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    // 创建 RollingFileAppender
    let roller = RollingFileAppender::builder().build("foo.log", Box::new(policy)).unwrap();

    // 创建日志配置
    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("default", Box::new(roller)))
        .build(Root::builder().appender("default").build(LevelFilter::Info))
        .unwrap();

    // 配置log4rs
    log4rs::init_config(config).unwrap();

    // 测试日志信息
    info!("Hello, log file!");
}