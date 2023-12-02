use crate::common::settings::Settings;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::info;
use ntex::web::types::State;
use r2d2::PooledConnection;

pub type ConnMng = ConnectionManager<PgConnection>;
pub type DbPool = r2d2::Pool<ConnMng>;

pub type PgConn = PooledConnection<ConnMng>;
pub fn init_pool(config: Settings) -> DbPool {
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let db_url = config.get_database_url();
    info!("db_url: {}", &db_url);

    let manager: ConnMng = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

pub fn get_conn(pool: State<DbPool>) -> PgConn {
    pool.get().expect("couldn't get db connection from pool")
}
