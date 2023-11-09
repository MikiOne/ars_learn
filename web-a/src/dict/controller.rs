use crate::common::DbPool;
use crate::dict;
use crate::models::LoginBO;
use log::{error, info, warn};
use ntex::web;
use ntex::web::HttpResponse;

#[web::get("/hf/dict/get/{dict_id}")]
pub async fn get_by_id(
    pool: web::types::State<DbPool>,
    dict_id: web::types::Path<i32>,
) -> Result<HttpResponse, web::Error> {
    let dict_id = dict_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let hf_dict = web::block(move || {
        let bo = LoginBO { code: "F10".to_string(), name: "工商局变更回执".to_string() };
        let res = dict::service::find_with_login(bo, &mut conn);
        info!("工商局变更回执 result: {:?}", res);
        dict::service::find_by_id(dict_id, &mut conn)
    })
    .await?;

    if let Some(hf_dict) = hf_dict {
        Ok(HttpResponse::Ok().json(&hf_dict))
    } else {
        let not_found = format!("No hf_dict found with id: {}", dict_id);
        error!("{}", &not_found);
        let res = HttpResponse::NotFound().body(not_found);
        Ok(res)
    }
}
