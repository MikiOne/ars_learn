use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schemas::wa_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Insertable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub pwd_hash: Option<String>,
    pub remark: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schemas::wa_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub pwd_hash: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schemas::wa_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}
