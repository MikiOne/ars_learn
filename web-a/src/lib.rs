use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

mod schema;
mod models;
mod hf_dict_dao;
pub mod hf_dict_ctrl;


pub type ConnMng = ConnectionManager<PgConnection>;
pub type DbPool = r2d2::Pool<ConnMng>;