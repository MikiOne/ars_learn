use crate::common::biz_code::BizCode;
use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebAppError {
    #[error("Request failed: {0:?}")]
    Custom(#[from] anyhow::Error),

    #[error("Request failed: {0:?}")]
    AuthError(BizCode),

    #[error("Request failed: {0}")]
    DieselError(#[from] diesel::result::Error),
}

impl WebResponseError for WebAppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            // WebAppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            WebAppError::Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }
}
