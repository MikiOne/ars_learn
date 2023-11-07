use ntex::web;
use ntex::web::HttpResponse;
use crate::{DbPool, hf_dict_dao};

#[web::get("/hf/dict/get/{dict_id}")]
pub async fn get_by_id(
    pool: web::types::State<DbPool>,
    dict_id: web::types::Path<i32>,
) -> Result<HttpResponse, web::Error> {
    let dict_id = dict_id.into_inner();
    let mut conn  = pool.get().expect("couldn't get db connection from pool");

    let hf_dict = web::block(move || hf_dict_dao::find_by_id(dict_id, &mut conn)).await?;

    if let Some(hf_dict) = hf_dict {
        Ok(HttpResponse::Ok().json(&hf_dict))
    } else {
        let res = HttpResponse::NotFound().body(format!("No hf_dict found with id: {}", dict_id));
        Ok(res)
    }
}