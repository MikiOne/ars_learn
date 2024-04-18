use log::info;
use std::{task::Context, task::Poll};

use ntex::http::body::{Body, BodySize, MessageBody, ResponseBody};
use ntex::util::{BoxFuture, Bytes};
use ntex::web::{Error, WebRequest, WebResponse};
use ntex::{Middleware, Service, ServiceCtx};
use serde_derive::Serialize;
use serde_json::Value;

pub struct ResponseData;

impl<S> Middleware<S> for ResponseData {
    type Service = ResponseDataMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        ResponseDataMiddleware { service }
    }
}

pub struct ResponseDataMiddleware<S> {
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for ResponseDataMiddleware<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: 'static,
{
    type Response = WebResponse;
    type Error = Error;
    type Future<'f> = BoxFuture<'f, Result<WebResponse, Error>> where Self: 'f;

    ntex::forward_poll_ready!(service);
    ntex::forward_poll_shutdown!(service);

    fn call<'a>(&'a self, req: WebRequest<Err>, ctx: ServiceCtx<'a, Self>) -> Self::Future<'a> {
        Box::pin(async move {
            let s2 = ctx.call(&self.service, req).await.map(|res| {
                let s1 = res.map_body(move |_, body| {
                    let ss = Body::from_message(RespBody::new(body)).into();
                    ss
                });
                s1
            });
            s2
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RespData {
    code: String,
    msg: String,
    data: Value,
}

pub struct RespBody {
    body: ResponseBody<Body>,
    bsize: BodySize,
}

impl RespBody {
    fn new(body: ResponseBody<Body>) -> RespBody {
        RespBody { bsize: (&body).size(), body }
    }

    fn set_size(&mut self, bsize: BodySize) {
        self.bsize = bsize;
    }

    fn get_size(&self) -> BodySize {
        self.bsize
    }
}

impl MessageBody for RespBody {
    /// 未完成，需要吧 "{ code: "000000".to_string(), msg: "Success".to_string(), data }"
    /// 对应的size赋值成功才行
    fn size(&self) -> BodySize {
        self.get_size()
    }

    fn poll_next_chunk(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Box<dyn std::error::Error>>>> {
        match self.body.poll_next_chunk(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                // self.body_accum.extend_from_slice(&chunk);
                // let ss = format!("{:?}", chunk);
                let input_bytes: &[u8] = chunk.as_ref();
                info!("chunk.input_bytes: {:?}", &input_bytes);
                let input_str = std::str::from_utf8(input_bytes).unwrap();
                let data: Value = serde_json::from_str(input_str).unwrap();
                info!("chunk.data: {:?}", data);

                let data =
                    RespData { code: "000000".to_string(), msg: "Success".to_string(), data };
                let json = ntex_bytes::Bytes::from(serde_json::to_string(&data)?);
                self.set_size(json.size());

                Poll::Ready(Some(Ok(json)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
