use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub mod consts;
pub mod error;
pub mod settings;

pub type ConnMng = ConnectionManager<PgConnection>;
pub type DbPool = r2d2::Pool<ConnMng>;
