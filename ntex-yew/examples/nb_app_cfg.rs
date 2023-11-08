use ntex::web;
use ntex::web::{get, head, HttpResponse, scope, ServiceConfig};

async fn index() -> impl web::Responder {
    "Hello world!"
}



fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/scoped")
            .route(get().to(|| async { HttpResponse::Ok().body("scoped test") }))
            .route(head().to(|| async { HttpResponse::MethodNotAllowed().finish() }))
    );
}

fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(get().to(|| async { HttpResponse::Ok().body("App") }))
            .route(get().to(|| async { HttpResponse::MethodNotAllowed().finish() }))
    );
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .configure(config)
            .service(scope("/api").configure(scoped_config))
            .route("/index.html", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}