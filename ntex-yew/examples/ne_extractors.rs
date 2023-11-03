use ntex::web;
use ntex::web::types::JsonConfig;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[web::get("/path/users/{user_id}/{friend}")] // <- define path parameters
async fn index_path(info: web::types::Path<Info>) -> Result<String, web::Error> {
    Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id))
}

#[web::get("/match/users/{user_id}/{friend}")] // <- define path parameters
async fn index_match_info(req: web::HttpRequest) -> Result<String, web::Error> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

#[derive(Deserialize)]
struct UserInfo {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[web::get("/query/users")]
async fn index_query(info: web::types::Query<UserInfo>) -> String {
    format!("Welcome {}!", info.username)
}

/// deserialize `Info` from request's body
#[web::post("/submit/users")]
async fn submit_users(info: web::types::Json<UserInfo>) -> Result<String, web::Error> {
    Ok(format!("Welcome {}!", info.username))
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index_payload(info: web::types::Json<UserInfo>) -> impl web::Responder {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
#[web::post("/form/data")]
async fn index_form(form: web::types::Form<FormData>) -> Result<String, web::Error> {
    Ok(format!("Welcome {}!", form.username))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        let json_config = JsonConfig::default().limit(4096);

        web::App::new()
            .service(index_path)
            .service(index_match_info)
            .service(index_query)
            .service(submit_users)
            .service(
                web::resource("/payload/users")
                    // change json extractor configuration
                    .state(json_config)
                    .route(web::post().to(index_payload)),
            )
            .service(index_form)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
