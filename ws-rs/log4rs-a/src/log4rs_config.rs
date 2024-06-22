use std::path::PathBuf;
use std::thread;

use anyhow::Result;
use log::{error, info, LevelFilter, warn};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use rand::distributions::Alphanumeric;
use rand::Rng;
use log4rs_a::{date_util, Env};


pub struct ConfigLog4rs(PathBuf);

// log config init
impl ConfigLog4rs {
    pub fn new(path_buf: &PathBuf) -> Result<ConfigLog4rs> {
        if let Env::Dev = Env::env()? {
            Ok(ConfigLog4rs(path_buf.join(Env::Dev.to_string())))
        } else {
            Ok(ConfigLog4rs(path_buf.to_owned()))
        }
    }

    // fn error(&self) -> Result<()> {
    //     println!(">>>>>>>>>test error");
    //     Err(anyhow!("test error"))
    // }
    // self.error().context("test context error")?;

    pub fn init_config(&self) -> Result<()> {
        let default_path = self.0.join("default.log");
        let error_path = self.0.join("error.log");

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[{d}] [{l}] [{P}:{T}:{I}] [{M}:{L}]:{m}{n}",
            )))
            .build();

        let default_rolling_appender = get_rolling_appender(default_path)?;
        let error_rolling_appender = get_rolling_appender(error_path)?;

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                    .build("default_log_file", Box::new(default_rolling_appender)),
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(LevelFilter::Error)))
                    .build("error_log_file", Box::new(error_rolling_appender)),
            )
            .logger(
                Logger::builder()
                    .appender("default_log_file")
                    .additive(false)
                    .build("default", LevelFilter::Info),
            )
            .logger(
                Logger::builder()
                    .appender("error_log_file")
                    .additive(false)
                    .build("error", LevelFilter::Error),
            )
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("error_log_file")
                    .appender("default_log_file")
                    .build(LevelFilter::Info),
            )?;

        match log4rs::init_config(config) {
            Ok(_) => info!("log4rs config success"),
            Err(err) => error!("log4rs config error: {}", err),
        }
        info!("current env: {:?}", Env::env());

        log_panics::init();
        Self::test_log_files();
        Ok(())
    }

    pub fn test_log_files() {
        thread::spawn(|| {
            for i in 0..30000000 {
                let msg: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(170)
                    .map(char::from)
                    .collect();
                let msg2: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(170)
                    .map(char::from)
                    .collect();

                info!("{}>>>>>>test info {}", i, msg2);
                warn!(
                    "{}>>>>>>test warn {}",
                    i,
                    rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(170)
                        .map(char::from)
                        .collect::<String>()
                );
                error!("{}>>>>>>test error {}", i, msg);
            }

            // panic!(">>>>>>>>>test panic");
        });
    }
}

fn get_rolling_appender(log_path: PathBuf) -> Result<RollingFileAppender> {
    let window_size = 30;
    let size_limit = 2;

    let size_m = byte_unit::n_mb_bytes!(size_limit) as u64;
    let size_trigger = SizeTrigger::new(size_m);
    let fixed_window_roller = FixedWindowRoller::builder().build(
        &format!(
            "{}-{}-{{}}.log.gz",
            log_path.display(),
            date_util::now_to_ymd()
        ),
        window_size,
    )?;

    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    Ok(RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d}] [{l}] [{P}:{T}:{I}] [{M}:{L}]:{m}{n}",
        )))
        .build(log_path, Box::new(compound_policy))?)
}
