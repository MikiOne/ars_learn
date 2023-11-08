use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::hf_dict)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Insertable)]
pub struct HfDict {
    pub id: i32,
    pub group: i16,
    pub code: Option<String>,
    pub name: Option<String>,
    pub remark: Option<String>,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginBO {
    pub code: String,
    pub name: String,
}
