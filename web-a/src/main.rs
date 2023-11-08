use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;
use ntex::web::{middleware, App, HttpServer};
use web_a::{hf_dict_ctrl, ConnMng, DbPool};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info, ntex=info,diesel=debug");
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
            .wrap(middleware::Logger::default())
            .service((hf_dict_ctrl::get_by_id))
    })
    .bind(&bind)?
    .run()
    .await
}
