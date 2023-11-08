use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub mod error;

pub type ConnMng = ConnectionManager<PgConnection>;
pub type DbPool = r2d2::Pool<ConnMng>;
