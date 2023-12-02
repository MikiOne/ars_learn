use std::env;

use log::info;
use ntex::web::{middleware, App, HttpServer};

use web_diesel_async::common::db_mg;
use web_diesel_async::common::db_mg::DbPool;
use web_diesel_async::common::settings::Settings;
use web_diesel_async::middleware::auth_filter;
use web_diesel_async::user::user_handler;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("读取配置文件出错");
    env::set_var("RUST_BACKTRACE", "1");

    if config.is_debug() {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info, ntex=info,diesel=debug");
    }

    env_logger::init();
    let pool: DbPool = db_mg::init_pool(config).await;

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
