use std::io;

use derive_more::{Display, Error};
use ntex::http::{Response, StatusCode};
use ntex::util::Buf;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use ntex::{http, web};
use ntex_files::NamedFile;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyErrStruct {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl web::error::WebResponseError for MyErrStruct {}

#[web::get("/myError")]
async fn my_err_struct() -> Result<&'static str, MyErrStruct> {
    Err(MyErrStruct { name: "test" })
}

#[web::get("/ioerror")]
async fn ioerror(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl web::error::WebResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

#[web::get("/err/enum")]
async fn enum_err() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}

#[web::get("/err/helper")]
async fn helper_err() -> Result<String, web::Error> {
    let result = Err(MyErrStruct { name: "test error" });

    // result.map_err(|err| web::error::ErrorBadRequest(err.name).into())
    result?
}

#[web::get("/err/helper2")]
async fn helper_err2() -> Result<String, MyErrStruct> {
    Err(MyErrStruct { name: "test error" })
}

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl WebResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

#[web::get("/err/user/err")]
async fn user_err() -> Result<String, web::Error> {
    let result = Err(UserError::ValidationError { field: "test error".to_string() });

    // result.map_err(|err| web::error::ErrorBadRequest(err.name).into())
    result?
}

#[derive(Debug, Display, Error)]
enum CommonError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl WebResponseError for CommonError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CommonError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // fn error_response(&self, _: &HttpRequest) -> HttpResponse {
    //     HttpResponse::build(self.status_code())
    //         .set_header("content-type", "text/html; charset=utf-8")
    //         .body(self.to_string())
    // }
}

#[web::get("/err/common")]
async fn common_err() -> Result<&'static str, CommonError> {
    do_thing_that_fails().map_err(|_e| CommonError::InternalError)?;
    Ok("success!")
}

fn do_thing_that_fails() -> io::Result<NamedFile> {
    // Ok(NamedFile::open("/Users/egal/cxy_mx/huifu/rsa/hf_rsa_public_key_2048.pub")?)
    Ok(NamedFile::open("static/index.html")?)
}

fn do_thing_that_fails2() -> Result<String, web::Error> {
    let result = Err(UserError::ValidationError { field: "test error".to_string() });
    result?
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    web::HttpServer::new(|| {
        web::App::new()
            .service(ioerror)
            .service(my_err_struct)
            .service(enum_err)
            .service(helper_err)
            .service(user_err)
            .service(common_err)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
