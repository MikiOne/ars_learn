use log::info;
use ntex::web;
use ntex::web::types::{Json, State};
use ntex::web::{HttpResponse, Responder, ServiceConfig};
use serde_json::json;

use crate::common::biz_code::BizCode;
use crate::common::biz_resp::RespData;
use crate::common::db_mg::{self, DbPool, PgConn};
use crate::middleware::auth_handler::LoggedUser;
use crate::user::user_models::NewUser;
use crate::user::user_repository;

/// 用户WEB api入口
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/user").service((get_user, create, logout)));
}

#[web::get("/info/{uid}")]
pub async fn get_user(pool: State<DbPool>, uid: web::types::Path<i32>) -> HttpResponse {
    let user_id = uid.into_inner();
    let mut conn: PgConn = db_mg::get_conn(pool).await;

    match user_repository::get_by_id(user_id, &mut conn).await {
        Ok(user) => RespData::success(user),
        Err(err) => RespData::from_biz_error(&err),
    }
}

#[web::post("/create")]
pub async fn create(
    pool: State<DbPool>,
    new_user: Json<NewUser>,
) -> Result<impl Responder, web::Error> {
    let mut conn: PgConn = db_mg::get_conn(pool).await;
    let user = user_repository::save_user(new_user.clone(), &mut conn).await?;

    let json = json!({"code": "000000","msg": "Success", "data": &user});
    Ok(HttpResponse::Ok().content_type("application/json").body(json))
}

#[web::post("/register")]
pub async fn register(
    pool: State<DbPool>,
    new_user: Json<NewUser>,
) -> Result<impl Responder, web::Error> {
    let mut conn: PgConn = db_mg::get_conn(pool).await;
    let user = user_repository::save_user(new_user.clone(), &mut conn).await?;

    let json = json!({"code": "000000","msg": "Success", "data": &user});
    Ok(HttpResponse::Ok().content_type("application/json").body(json))
}

// login

// let token = jwt_handler::create_jwt(res.id.to_string(), &Role::User)?;
// info!("login token info: {:?}", token);
// Ok(HttpResponse::Ok().content_type("application/json").json(&token))

// #[web::post("/user/login")]
// pub async fn login(
//     pool: State<DbPool>,
//     bo: Json<LoginUser>,
// ) -> Result<impl Responder, WebAppError> {
//     let mut conn = pool.get().expect("couldn't get db connection from pool");
//
//     if let Some(res) = dict::service::find_with_login(bo.clone(), &mut conn)? {
//         let token = jwt_handler::create_jwt(res.id.to_string(), &Role::User)?;
//         info!("login token info: {:?}", token);
//
//         Ok(HttpResponse::Ok().content_type("application/json").json(&token))
//     } else {
//         let json = json!({"code": "400","msg": "用户不存在"});
//         Ok(HttpResponse::Ok().content_type("application/json").body(json))
//     }
// }

#[web::get("/logout")]
async fn logout(user: LoggedUser) -> HttpResponse {
    info!(">>>>>>>>>>>>>LoggedUser: {:?}", user);
    RespData::with_biz_code(BizCode::LOGIN_TIMEOUT)
}
