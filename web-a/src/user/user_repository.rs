use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl, SelectableHelper,
};

use crate::common::error::WebAppError;
use crate::user::user_models;
use crate::user::user_models::{NewUser, SimpleUser};

pub fn get_by_id(
    user_id: i32,
    conn: &mut PgConnection,
) -> Result<Option<user_models::User>, WebAppError> {
    use crate::schemas::wa_users::dsl::*;
    if user_id != 1 {
        Err(WebAppError::DieselError(diesel::result::Error::NotFound))
    } else {
        Ok(wa_users.find(user_id).first::<user_models::User>(conn).optional()?)
    }
}

pub fn save_user(new_user: NewUser, conn: &mut PgConnection) -> Result<SimpleUser, WebAppError> {
    use crate::schemas::wa_users::dsl::*;

    let res_user = diesel::insert_into(wa_users)
        .values(&new_user)
        .returning(SimpleUser::as_returning())
        .get_result(conn)?;

    Ok(res_user)
}

pub fn get_by_name(
    user_name: String,
    conn: &mut PgConnection,
) -> Result<Option<user_models::User>, WebAppError> {
    use crate::schemas::wa_users::dsl::*;
    Ok(wa_users.filter(name.eq(user_name)).first::<user_models::User>(conn).optional()?)
}

pub fn get_by_name_email(
    _email: String,
    _name: String,
    conn: &mut PgConnection,
) -> Result<Option<user_models::User>, WebAppError> {
    use crate::schemas::wa_users::dsl::*;
    let res = wa_users
        .filter(name.eq(_name).and(email.eq(_email)))
        .first::<user_models::User>(conn)
        .optional()?;
    Ok(res)
}
