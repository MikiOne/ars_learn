#![allow(dead_code, clippy::type_complexity)]

use futures::future::{ok, Either, Ready};
use ntex::http;
use ntex::service::{Middleware, Service, ServiceCall, ServiceCtx};
use ntex::web::{Error, ErrorRenderer, HttpResponse, WebRequest, WebResponse};

use ntex::web;

pub struct CheckLogin;

impl<S> Middleware<S> for CheckLogin {
    type Service = CheckLoginMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        CheckLoginMiddleware { service }
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for CheckLoginMiddleware<S>
where
    Err: 'static,
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;
    type Future<'f> = Either<ServiceCall<'f, S, WebRequest<Err>>, Ready<Result<Self::Response, Self::Error>>> where Self: 'f, Err: 'f;

    ntex::forward_poll_ready!(service);
    ntex::forward_poll_shutdown!(service);

    fn call<'a>(&'a self, req: WebRequest<Err>, ctx: ServiceCtx<'a, Self>) -> Self::Future<'a> {
        // We only need to hook into the `start` for this middleware.

        let is_logged_in = false; // Change this to see the change in outcome in the browser

        if is_logged_in {
            Either::Left(ctx.call(&self.service, req))
        } else {
            // Don't forward to /login if we are already on /login
            if req.path() == "/login" {
                Either::Left(ctx.call(&self.service, req))
            } else {
                Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/login")
                        .finish()
                        .into_body(),
                )))
            }
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    web::server(|| {
        web::App::new()
            .filter(|req: web::WebRequest<_>| async move {
                println!("Hi from start. You requested: {}", req.path());
                Ok(req)
            })
            .wrap(CheckLogin)
            .service(web::resource("/login").to(|| async {
                // "You are on /login. Go to src/redirect.rs to change this behavior."
                "你还未登录请登录后再使用"
            }))
            .service(
                web::resource("/").to(|| async {
                    "Hello, middleware! Check the console where the server is run."
                }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
