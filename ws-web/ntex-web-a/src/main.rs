use std::env;

use log::info;
use ntex::web::{middleware, App, HttpServer};

use ntex_web_a::common::db_mg;
use ntex_web_a::common::db_mg::DbPool;
use ntex_web_a::common::settings::Settings;
use ntex_web_a::middleware::auth_filter;
use ntex_web_a::user::user_handler;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("读取配置文件出错");

    env::set_var("RUST_BACKTRACE", "1");
    // env::set_var("RUST_LOG", "info, ntex=info,diesel=debug");
    if config.is_debug() {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    let pool: DbPool = db_mg::init_pool(config);

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .state(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(auth_filter::JwtFilter)
            // .wrap(json_result::RespData)
            // .wrap(resp_data::ResponseData)
            .configure(user_handler::config)
        // .service((user::login, user::logout))
    })
    .bind(&bind)?
    .run()
    .await
}
