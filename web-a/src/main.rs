use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::{error, info};
use ntex::web::{middleware, App, HttpServer};

use web_a::common::settings::Settings;
use web_a::common::{ConnMng, DbPool};
use web_a::dict::controller;
use web_a::middleware::auth_filter;
use web_a::user;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("读取配置文件出错");

    env::set_var("RUST_BACKTRACE", "1");
    // env::set_var("RUST_LOG", "info, ntex=info,diesel=debug");
    if config.is_debug() {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let db_url = config.get_database_url();
    info!("db_url: {}", &db_url);

    let manager: ConnMng = ConnectionManager::<PgConnection>::new(db_url);
    let pool: DbPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .state(pool.clone())
            // .filter(|req: WebRequest<_>| async move {
            //     info!("Hi from start. You requested: {}", req.path());
            //     // auth::authorize((req.headers()));
            //     let jwt_str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIyOCIsInJvbGUiOiJVc2VyIiwiZXhwIjoxNjk5NTA4NzQ5fQ.15F-bQPelGT_hWeXENgl-jzZ8PcAXw91vwsCiJO5qLNpOAaJ8rdpWSJPy7bgaFREJGaFURVTvbjSdZfDcec5gg".to_string();
            //     let res = jwt::decode_jwt(jwt_str);
            //     info!("auth::decode_jwt(jwt_str): {:?}", res);
            //     if let Some(token) = req.headers().get("Token") {
            //         // 处理获取到的 token 值
            //         println!("Token value: {:?}", token);
            //     } else {
            //         // 如果没有找到 "Token" 头部，则返回错误或默认值
            //         // return Err(Error::from_str(404, "Token header not found"));
            //     }
            //     Ok(req)
            // })
            .wrap(middleware::Logger::default())
            .wrap(auth_filter::JwtFilter)
            .service((controller::get_by_id, user::login, user::logout))
    })
    .bind(&bind)?
    .run()
    .await
}
