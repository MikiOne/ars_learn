use ntex::web::{self, App, HttpResponse, HttpServer};

#[web::get("/")]
async fn hello() -> impl web::Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
