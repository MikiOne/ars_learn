use crate::diesel_ns::{actions, models};
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use ntex::web;
use ntex::web::HttpResponse;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Finds user by UID.
#[web::get("/user/{user_id}")]
pub async fn get_user(
    pool: web::types::State<DbPool>,
    user_uid: web::types::Path<Uuid>,
) -> Result<HttpResponse, web::Error> {
    let user_uid = user_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &mut conn)).await?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(&user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[web::post("/user")]
pub async fn add_user(
    pool: web::types::State<DbPool>,
    form: web::types::Json<models::NewUser>,
) -> Result<HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.name, &mut conn)).await?;

    Ok(HttpResponse::Ok().json(&user))
}
