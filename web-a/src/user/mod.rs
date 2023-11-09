use log::info;
use ntex::web;
use ntex::web::types::{Json, State};
use ntex::web::{HttpResponse, Responder};
use serde_json::json;

use crate::auth::jwt_handler;
use crate::auth::models::Role;
use crate::common::error::WebAppError;
use crate::common::DbPool;
use crate::dict;
use crate::middleware::auth_handler::LoggedUser;
use crate::models::LoginBO;

#[web::post("/user/login")]
pub async fn login(pool: State<DbPool>, bo: Json<LoginBO>) -> Result<impl Responder, WebAppError> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    if let Some(res) = dict::service::find_with_login(bo.clone(), &mut conn)? {
        let token = jwt_handler::create_jwt(res.id.to_string(), &Role::User)?;
        info!("login token info: {:?}", token);

        Ok(HttpResponse::Ok().content_type("application/json").json(&token))
    } else {
        let json = json!({"code": "400","msg": "用户不存在"});
        Ok(HttpResponse::Ok().content_type("application/json").body(json))
    }
}

#[web::get("/user/logout")]
async fn logout(user: LoggedUser) -> HttpResponse {
    info!(">>>>>>>>>>>>>LoggedUser: {:?}", user);
    let json = json!({"code": "000","msg": "成功登出"});
    HttpResponse::Ok().content_type("application/json").body(json)
}
