use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use std::env;

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body("{\"id\":\"00000000-0000-0000-0000-000000000000\",\"jsonrpc\":\"2.0\",\"result\":{\"greeting\":\"Hello, Alice!\"}}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_url = env::var("CLIENT_URL").expect("You must set CLIENT_URL");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&client_url)
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/api", web::post().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
