use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("Request failed: {0:?}")]
    Custom(#[from] anyhow::Error),

    #[error("Request failed: {0:?}")]
    DieselError(#[from] diesel::result::Error),
}

impl WebResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::WrongCredentialsError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JWTTokenError => StatusCode::from_u16(401).unwrap(),
            AppError::JWTTokenCreationError => StatusCode::UNAUTHORIZED,
            AppError::NoAuthHeaderError => StatusCode::UNAUTHORIZED,
            AppError::InvalidAuthHeaderError => StatusCode::UNAUTHORIZED,
            AppError::NoPermissionError => StatusCode::UNAUTHORIZED,
            AppError::Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// struct ErrorResponse {
//     message: String,
//     status: String,
// }

// impl warp::reject::Reject for Error {}

// pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
//     let (code, message) = if err.is_not_found() {
//         (StatusCode::NOT_FOUND, "Not Found".to_string())
//     } else if let Some(e) = err.find::<Error>() {
//         match e {
//             Error::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
//             Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
//             Error::JWTTokenCreationError => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Internal Server Error".to_string(),
//             ),
//             Error::NoAuthHeaderError => (StatusCode::BAD_REQUEST, e.to_string()),
//             Error::InvalidAuthHeaderError => (StatusCode::BAD_REQUEST, e.to_string()),
//             Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
//         }
//     } else if err.find::<reject::MethodNotAllowed>().is_some() {
//         (
//             StatusCode::METHOD_NOT_ALLOWED,
//             "Method Not Allowed".to_string(),
//         )
//     } else {
//         eprintln!("unhandled rejection: {:?}", err);
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             "Internal Server Error".to_string(),
//         )
//     };
//
//     let json = warp::reply::json(&ErrorResponse {
//         message,
//         status: code.to_string(),
//     });
//     Ok(warp::reply::with_status(json, code))
// }
