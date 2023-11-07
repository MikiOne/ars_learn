use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use log::info;
use ntex::web;
use ntex::web::middleware;

use crate::diesel_ns::controller;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod diesel_ns;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "ntex=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    info!("db_url: {}",&db_url);

    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool: DbPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    web::HttpServer::new(move || {
        web::App::new()
            .state(pool.clone())
            .wrap(middleware::Logger::default())
            .service((controller::get_user, controller::add_user))
    })
    .bind(&bind)?
    .run()
    .await
}
