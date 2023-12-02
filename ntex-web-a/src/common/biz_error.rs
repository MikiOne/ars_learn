use derive_more::Display;
use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use std::fmt::{Debug, Display, Formatter};

use crate::common::biz_code::BizCode;
use crate::common::biz_resp::RespData;

// #[derive(Debug, Copy, Clone, Display)]
// pub struct BizError(pub BizCode);
// impl BizError {
//     // pub fn with_err(biz_code: BizCode, err_str: String) -> Self {
//     //     BizError { biz_err: BizError(biz_code), err_str }
//     // }
//
//     // pub fn with_err(biz_code: BizCode, err: Box<dyn std::error::Error>) -> Self {
//     //     BizError { biz_code, err_str: Some(format!("{}", err)) }
//     // }
// }
// impl WebResponseError for BizError {
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             _ => StatusCode::OK,
//         }
//     }
//
//     fn error_response(&self, _: &HttpRequest) -> HttpResponse {
//         RespData::with_biz_code(self.0)
//     }
// }

pub struct BizError {
    pub biz_code: BizCode,
    err_str: String,
}
impl BizError {
    pub fn new(biz_code: BizCode) -> Self {
        BizError { biz_code, err_str: "".to_string() }
    }
    pub fn with_err(biz_code: BizCode, err_str: String) -> Self {
        BizError { biz_code, err_str }
    }

    // pub fn with_err(biz_code: BizCode, err: Box<dyn std::error::Error>) -> Self {
    //     BizError { biz_code, err_str: Some(format!("{}", err)) }
    // }

    pub fn code_reason(&self) -> String {
        self.biz_code.code_reason()
    }
}

impl Display for BizError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.biz_code.reason().unwrap(), self.err_str)
    }
}

impl Debug for BizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code_reason(), self.err_str)
    }
}

impl WebResponseError for BizError {
    fn status_code(&self) -> StatusCode {
        match *self {
            _ => StatusCode::OK,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        RespData::from_biz_error(&self)
    }
}
