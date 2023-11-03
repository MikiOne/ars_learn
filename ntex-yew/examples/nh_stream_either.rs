use futures::{future::ok, stream::once};
use ntex::util::Bytes;
use ntex::util::Either;
use ntex::web;

type RegisterResult = Either<web::HttpResponse, Result<&'static str, web::Error>>;

fn is_a_variant() -> bool {
    false
}

#[web::get("/either")]
async fn either_index() -> RegisterResult {
    if is_a_variant() {
        // choose Left variant
        Either::Left(web::HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        Either::Right(Ok("Hello!"))
    }
}

#[web::get("/stream")]
async fn stream() -> web::HttpResponse {
    let body = once(ok::<_, web::Error>(Bytes::from_static(b"test")));

    web::HttpResponse::Ok().content_type("application/json").streaming(body)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(stream).service(either_index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
