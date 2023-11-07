use crate::diesel_ns::models;
use diesel::prelude::*;
use uuid::Uuid;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &mut SqliteConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::diesel_ns::schema::users::dsl::*;

    let user = users.filter(id.eq(uid.to_string())).first::<models::User>(conn).optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &mut SqliteConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Ntex to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::diesel_ns::schema::users::dsl::*;

    let new_user = models::User { id: Uuid::new_v4().to_string(), name: nm.to_owned() };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}
