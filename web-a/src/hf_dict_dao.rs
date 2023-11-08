use crate::models;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub fn find_by_id(
    dict_id: i32,
    conn: &mut PgConnection,
) -> Result<Option<models::HfDict>, diesel::result::Error> {
    use crate::schema::hf_dict::dsl::*;

    let res = hf_dict.find(dict_id).first::<models::HfDict>(conn).optional()?;
    Ok(res)
}
