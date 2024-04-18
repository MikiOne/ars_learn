// use crate::common::error::WebAppError;
// use diesel::{
//     BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
//     RunQueryDsl,
// };
//
// use crate::user::user_models;
// use crate::user::user_models::LoginBO;
//
// pub fn find_by_id(
//     dict_id: i32,
//     conn: &mut PgConnection,
// ) -> Result<Option<user_models::HfDict>, diesel::result::Error> {
//     use crate::dict::schema::hf_dict::dsl::*;
//
//     let res = hf_dict.find(dict_id).first::<user_models::HfDict>(conn).optional()?;
//     Ok(res)
// }
//
// pub fn find_with_login(
//     bo: LoginBO,
//     conn: &mut PgConnection,
// ) -> Result<Option<user_models::HfDict>, WebAppError> {
//     use crate::dict::schema::hf_dict::dsl::*;
//     let res = hf_dict
//         .filter(name.eq(bo.name).and(code.eq(bo.code)))
//         .first::<user_models::HfDict>(conn)
//         .optional()?;
//     Ok(res)
// }
//
// // let users = diesel::sql_query("SELECT * FROM users WHERE id > ? AND name <> ?");
// // let users = users
// //     .bind::<Integer, _>(1)
// //     .bind::<Text, _>("Tess")
// //     .get_results(conn);
//
// // let sql = "SELECT * FROM hf_dict WHERE code = $1 and name = $2";
// // diesel::sql_query(sql)
// //     .bind::<Text, _>("bo.code")
// //     .bind::<Text, _>("bo.name")
// //     .get_result(conn)
// //     .optional()?;
