use env_logger::Env;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::util::BoxFuture;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Logger::default())
            .wrap(web::middleware::Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

impl<S> Middleware<S> for SayHi {
    type Service = SayHiMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        SayHiMiddleware { service }
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for SayHiMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;
    type Future<'f> = BoxFuture<'f, Result<Self::Response, Self::Error>> where Self: 'f;

    ntex::forward_poll_ready!(service);

    fn call<'a>(
        &'a self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'a, Self>,
    ) -> Self::Future<'_> {
        println!("Hi from start. You requested: {}", req.path());

        let fut = ctx.call(&self.service, req);
        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
