use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder, Result};
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
#[serde(untagged)]
pub enum GreetingJsonRpcResponse {
    MethodNotFoundError {
        error: MethodNotFoundError,
        id: String,
        jsonrpc: String,
    },
    GreetingResult {
        id: String,
        jsonrpc: String,
        result: GreetingResult,
    },
}

#[derive(Serialize)]
pub struct GreetingResult {
    greeting: String,
}

#[derive(Serialize)]
pub struct MethodNotFoundData {
    method: String,
}

#[derive(Serialize)]
pub struct MethodNotFoundError {
    code: i32,
    data: MethodNotFoundData,
    message: String,
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn index(request: web::Json<GreetingJsonRpcRequest>) -> Result<impl Responder> {
    if request.method != "greeting" {
        return Ok(web::Json(GreetingJsonRpcResponse::MethodNotFoundError {
            error: MethodNotFoundError {
                code: -32601,
                data: MethodNotFoundData {
                    method: request.method.clone(),
                },
                message: "Method not found".to_owned(),
            },
            id: request.id.clone(),
            jsonrpc: request.jsonrpc.clone(),
        }));
    }

    let name = request.params.name.trim();

    if name.is_empty() {
        return Ok(web::Json(GreetingJsonRpcResponse::GreetingResult {
            id: request.id.clone(),
            jsonrpc: request.jsonrpc.clone(),
            result: GreetingResult {
                greeting: "Hello, World!".to_owned(),
            },
        }));
    }

    Ok(web::Json(GreetingJsonRpcResponse::GreetingResult {
        id: request.id.clone(),
        jsonrpc: request.jsonrpc.clone(),
        result: GreetingResult {
            greeting: format!("Hello, {}!", name),
        },
    }))
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
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
