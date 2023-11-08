use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub mod hf_dict_ctrl;
mod hf_dict_dao;
mod models;
mod schema;

pub type ConnMng = ConnectionManager<PgConnection>;
pub type DbPool = r2d2::Pool<ConnMng>;
