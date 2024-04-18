use futures::future;
use futures::future::{Either, Ready};
use ntex::web::{Error, HttpResponse, WebRequest, WebResponse};
use ntex::{Middleware, Service, ServiceCall, ServiceCtx};
use serde_json::json;

pub struct RespData;

impl<S> Middleware<S> for RespData {
    type Service = RespDataMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        RespDataMiddleware { service }
    }
}

pub struct RespDataMiddleware<S> {
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for RespDataMiddleware<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: 'static,
{
    type Response = WebResponse;
    type Error = Error;
    type Future<'f> = Either<ServiceCall<'f, S, WebRequest<Err>>,
        Ready<Result<Self::Response, Self::Error>>> where Self: 'f, Err: 'f;

    ntex::forward_poll_ready!(service);
    ntex::forward_poll_shutdown!(service);

    fn call<'a>(&'a self, req: WebRequest<Err>, ctx: ServiceCtx<'a, Self>) -> Self::Future<'a> {
        // Box::pin(async move {
        //     let s2 = ctx.call(&self.service, req).await.map(|res| {
        //         let s1 = res.map_body(move |_, body| {
        //             let ss =
        //                 Body::from_message(BodyLogger { body, body_accum: BytesMut::new() }).into();
        //             ss
        //         });
        //         s1
        //     });
        //     s2
        // })

        let json = json!({"code": "0004","msg": "response data test"});
        Either::Right(future::ok(
            req.into_response(HttpResponse::Ok().content_type("application/json").body(json)),
        ))
    }
}
