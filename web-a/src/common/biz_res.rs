use log::error;
use ntex::http::error::BlockingError;
use ntex::web;
use ntex::web::{HttpResponse, WebResponseError};
use serde_derive::Serialize;

use crate::common::biz_code::BizCode;
use crate::common::biz_err::BizError;
use crate::common::error::WebAppError;

pub type BizResult<T> = Result<T, BizError>;
// pub type BizResExt<T> = Result<T, dyn WebResponseError>;

#[derive(Debug, Clone, Serialize)]
pub struct RespData<T> {
    code: String,
    msg: String,
    data: T,
}

impl<T> RespData<T> {
    fn with_success(data: T) -> Self {
        let biz_code = BizCode::SUCCESS;
        let msg = biz_code.reason().unwrap().to_string();
        RespData { code: biz_code.code().to_string(), msg, data }
    }

    pub fn success(data: T) -> HttpResponse
    where
        T: serde::Serialize,
    {
        let res_data = RespData::with_success(data);
        HttpResponse::Ok().json(&res_data)
    }
}

impl RespData<()> {
    pub fn with_biz_code(biz_code: BizCode) -> HttpResponse {
        let msg = biz_code.reason().unwrap().to_string();
        let resp_data = RespData { code: biz_code.code().to_string(), msg, data: {} };
        HttpResponse::Ok().json(&resp_data)
    }

    pub fn with_biz_code_err(err: &String, biz_code: BizCode) -> HttpResponse {
        let msg = biz_code.reason().unwrap().to_string();
        let err_msg = format!("{} {}", msg, err);
        let resp_data = RespData { code: biz_code.code().to_string(), msg: err_msg, data: {} };
        HttpResponse::Ok().json(&resp_data)
    }

    pub fn from_biz_error(biz_error: &BizError) -> HttpResponse {
        let resp_data =
            RespData { code: biz_error.biz_code.to_string(), msg: biz_error.to_string(), data: {} };
        HttpResponse::Ok().json(&resp_data)
    }

    // pub fn failure(err: String) -> HttpResponse {
    //     let res_data = RespData { code: "000030".to_string(), msg: err, data: () };
    //     HttpResponse::NotFound().json(&res_data)
    // }

    // pub fn with_err(err: WebAppError) -> Result<HttpResponse, web::Error> {
    //     let res_data = RespData { code: "000030".to_string(), msg: err.to_string(), data: () };
    //     Ok(HttpResponse::build(err.status_code()).json(&res_data))
    // }

    pub fn with_blocking_err(blocking_err: BlockingError<BizError>) -> HttpResponse {
        match blocking_err {
            BlockingError::Error(biz_error) => RespData::from_biz_error(&biz_error),
            err => {
                error!("Wet block blockingError: {}", err);
                RespData::with_biz_code_err(&err.to_string(), BizCode::SYSTEM_ERROR)
            }
        }
    }
}
