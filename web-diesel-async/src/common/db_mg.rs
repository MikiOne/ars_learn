use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use log::info;
use ntex::web::types::State;

use crate::common::settings::Settings;

pub type ConnMng = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbPool = bb8::Pool<ConnMng>;

pub type PgConn = bb8::PooledConnection<'static, ConnMng>;

pub async fn init_pool(config: Settings) -> DbPool {
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let db_url = config.get_database_url();
    info!("db_url: {}", &db_url);

    // set up connection pool
    let config: ConnMng = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    bb8::Pool::builder().build(config).await.unwrap()
}

pub async fn get_conn(pool: State<DbPool>) -> PgConn {
    pool.get_owned().await.expect("couldn't get db connection from pool")
}
