use ntex::http::error::BlockingError;
use ntex::web;
use ntex::web::{HttpResponse, WebResponseError};
use serde_derive::Serialize;

use crate::common::biz_code::BizCode;
use crate::common::error::WebAppError;

#[derive(Debug, Clone, Serialize)]
pub struct RespData<T> {
    code: String,
    msg: String,
    data: T,
}

impl<T> RespData<T> {
    fn with_success(data: T) -> Self {
        let biz_code = BizCode::SUCCESS;
        let msg = biz_code.desc().unwrap().to_string();
        RespData { code: biz_code.code().to_string(), msg, data }
    }

    pub fn success(data: T) -> Result<HttpResponse, web::Error>
    where
        T: serde::Serialize,
    {
        let res_data = RespData::with_success(data);
        Ok(HttpResponse::Ok().json(&res_data))
    }
}

impl RespData<()> {
    pub fn failure(err: String) -> Result<HttpResponse, web::Error> {
        let res_data = RespData { code: "000030".to_string(), msg: err, data: () };
        Ok(HttpResponse::NotFound().json(&res_data))
    }

    pub fn with_err(err: WebAppError) -> Result<HttpResponse, web::Error> {
        let res_data = RespData { code: "000030".to_string(), msg: err.to_string(), data: () };
        Ok(HttpResponse::build(err.status_code()).json(&res_data))
    }

    pub fn with_blocking_err(err: BlockingError<WebAppError>) -> Result<HttpResponse, web::Error> {
        let res_data = RespData { code: "000030".to_string(), msg: err.to_string(), data: () };
        Ok(HttpResponse::build(err.status_code()).json(&res_data))
    }
}
