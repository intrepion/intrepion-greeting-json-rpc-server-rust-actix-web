use actix_cors::Cors;
use actix_web::{
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    web, App, HttpResponse, HttpServer, Responder, Result,
};
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

pub async fn env_vars() -> HttpResponse {
    let client_url = env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:8080".to_owned());
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Env Vars</title>
    </head>
    <body>
        <p>Client URL: <a href="{client_url}">{client_url}</a></p>
    </body>
</html>"#
        ))
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
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let client_url = env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:8080".to_owned());
    println!("Base URL: {base_url}");
    println!("Client URL: {client_url}");
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
            .route("/", web::get().to(env_vars))
            .route("/", web::post().to(index))
            .route("/health_check", web::get().to(health_check))
    })
    .bind((base_url, 8080))?
    .run()
    .await
}
