use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct GreetingJsonRpcParams {
    name: String,
}

#[derive(Deserialize)]
struct GreetingJsonRpcRequest {
    id: String,
    jsonrpc: String,
    method: String,
    params: GreetingJsonRpcParams,
}

#[derive(Serialize)]
struct GreetingJsonRpcResponse {
    id: String,
    jsonrpc: String,
    result: GreetingJsonRpcResult,
}

#[derive(Serialize)]
struct GreetingJsonRpcResult {
    greeting: String,
}

async fn index(request: web::Json<GreetingJsonRpcRequest>) -> Result<impl Responder> {
    let greeting = GreetingJsonRpcResponse {
        id: request.id.clone(),
        jsonrpc: request.jsonrpc.clone(),
        result: GreetingJsonRpcResult {
            greeting: format!("Hello, {}!", request.params.name),
        },
    };
    Ok(web::Json(greeting))
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
