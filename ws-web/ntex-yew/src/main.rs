use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use log::info;
use ntex::web;
use ntex::web::middleware;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "ntex=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool: DbPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "127.0.0.1:8030";
    info!("Starting server at: {}", &bind);

    web::HttpServer::new(move || {
        web::App::new().state(pool.clone()).wrap(middleware::Logger::default())
        // .service((get_user))
    })
    .bind(&bind)?
    .run()
    .await
}
