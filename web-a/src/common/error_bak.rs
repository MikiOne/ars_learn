use crate::auth::error::AuthError;
use derive_more::Display;
use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use serde_derive::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum WebAppError {
    #[display(fmt = "Request failed: {error}")]
    Custom { code: String, error: anyhow::Error },

    #[display(fmt = "Request failed: {error}")]
    DieselError { code: String, error: diesel::result::Error },

    #[display(fmt = "Authorization failed: {error}")]
    AuthError { code: String, error: AuthError },
}

impl WebResponseError for WebAppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            WebAppError::AuthError { .. } => StatusCode::UNAUTHORIZED,
            WebAppError::Custom { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        // let err_json = serde_json::to_string_pretty(self);
        // HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(&err_json)
        // HttpResponse::Ok().json()
        // web::types::Json(
        //     RespData { code: "Name".to_string() }
        // ).with_header("x-version", "1.2.3")
        todo!()
    }

    // fn error_response(&self, _: &HttpRequest) -> HttpResponse {
    //     HttpResponse::build(self.status_code())
    //         .set_header("content-type", "text/html; charset=utf-8")
    //         .body(self.to_string())
    // }
}
