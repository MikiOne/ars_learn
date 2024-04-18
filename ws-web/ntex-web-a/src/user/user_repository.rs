use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use log::error;

use crate::common::biz_code::BizCode;
use crate::common::biz_error::BizError;
use crate::common::biz_resp::BizResult;
use crate::user::user_models::{NewUser, SimpleUser, User};

pub fn get_by_id(user_id: i32, conn: &mut PgConnection) -> BizResult<User> {
    use crate::schemas::wa_users::dsl::*;
    if user_id != 1 {
        let die_err = diesel::result::Error::NotFound;
        // Err(BizError(BizCode::USER_NOT_FOUND))
        Err(BizError::with_err(BizCode::DIESEL_ERROR, die_err.to_string()))
    } else {
        wa_users.find(user_id).first::<User>(conn).map_err(|err| {
            error!("Query user by id[{}] error: {:?}", user_id, err);
            BizError::with_err(BizCode::DIESEL_ERROR, err.to_string())
        })
    }
}

pub fn save_user(new_user: NewUser, conn: &mut PgConnection) -> BizResult<SimpleUser> {
    use crate::schemas::wa_users::dsl::*;

    diesel::insert_into(wa_users)
        .values(&new_user)
        .returning(SimpleUser::as_returning())
        .get_result(conn)
        .map_err(|err| {
            error!("Save user error: {:?}", err);
            BizError::with_err(BizCode::DIESEL_ERROR, err.to_string())
        })
}

// pub fn get_by_name(
//     user_name: String,
//     conn: &mut PgConnection,
// ) -> Result<Option<user_models::User>, BizError> {
//     use crate::schemas::wa_users::dsl::*;
//     Ok(wa_users.filter(name.eq(user_name)).first::<user_models::User>(conn).optional()?)
// }
//
// pub fn get_by_name_email(
//     _email: String,
//     _name: String,
//     conn: &mut PgConnection,
// ) -> Result<Option<user_models::User>, BizError> {
//     use crate::schemas::wa_users::dsl::*;
//     let res = wa_users
//         .filter(name.eq(_name).and(email.eq(_email)))
//         .first::<user_models::User>(conn)
//         .optional()?;
//     Ok(res)
// }
