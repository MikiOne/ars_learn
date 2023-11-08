use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;
use ntex::web::{middleware, App, HttpServer, WebRequest};

use web_a::common::{ConnMng, DbPool};
use web_a::dict::controller;
use web_a::user;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    // env::set_var("RUST_LOG", "info, ntex=info,diesel=debug");
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    info!("db_url: {}", &db_url);

    let manager: ConnMng = ConnectionManager::<PgConnection>::new(db_url);
    let pool: DbPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .state(pool.clone())
            .filter(|req: WebRequest<_>| async move {
                println!("Hi from start. You requested: {}", req.path());
                Ok(req)
            })
            .wrap(middleware::Logger::default())
            .service((controller::get_by_id, user::user_login))
    })
    .bind(&bind)?
    .run()
    .await
}
