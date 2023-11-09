use log::info;
use ntex::web;
use ntex::web::types::{Json, State};
use ntex::web::{HttpResponse, Responder};
use serde_json::json;

use crate::auth::Role;
use crate::common::error::AppError;
use crate::common::DbPool;
use crate::models::LoginBO;
use crate::{auth, dict};

async fn index(id: i32) -> String {
    format!("Hello {}", id)
}

#[web::post("/user/login")]
pub async fn user_login(
    pool: State<DbPool>,
    bo: Json<LoginBO>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    if let Some(res) = dict::service::find_with_login(bo.clone(), &mut conn)? {
        let token = auth::create_jwt(res.id.to_string(), &Role::User)?;
        info!("login token info: {:?}", token);

        Ok(HttpResponse::Ok().content_type("application/json").json(&token))
    } else {
        let json = json!({"code": "400","msg": "用户不存在"});
        Ok(HttpResponse::Ok().content_type("application/json").body(json))
    }
}

// async fn logout(id: Identity) -> HttpResponse {
//     id.forget();
//     HttpResponse::Found().header("location", "/").finish()
// }
