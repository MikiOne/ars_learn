// use derive_more::Display;
// use ntex::http::StatusCode;
// use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
//
// use crate::common::biz_code::BizCode;
// use crate::common::biz_res::RespData;
//
// #[derive(Debug, Copy, Clone, Display)]
// pub struct BizError {
//     biz_code: BizCode,
//     err_str: String,
// }
//
// impl BizError {
//     pub fn with_err(biz_code: BizCode, err: Box<dyn std::error::Error>) -> Self {
//         BizError { biz_code, err_str: Some(format!("{}", err)) }
//     }
// }
// // impl WebResponseError for BizError {
// //     fn status_code(&self) -> StatusCode {
// //         match *self {
// //             _ => StatusCode::OK,
// //         }
// //     }
// //
// //     fn error_response(&self, _: &HttpRequest) -> HttpResponse {
// //         RespData::with_biz_code(self.0)
// //     }
// // }
