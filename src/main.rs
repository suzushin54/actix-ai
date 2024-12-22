use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/status", web::get().to(status))
        .route("/chat", web::post().to(chat))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn status() -> impl Responder {
    HttpResponse::Ok().body("running!")
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

async fn chat(req: web::Json<ChatRequest>) -> impl Responder {
    let message = &req.message;
    let res = ai_interface(message);

    HttpResponse::Ok().json(ChatResponse {
        response: res
    })
}

fn ai_interface(input: &str) -> String {
    let mut ress = HashMap::new();
    ress.insert("Hello", "Hi! What's up?");
    ress.insert("Good bye", "See you tomorrow!");

    for (key, value) in ress {
        if input.contains(key) {
            return value.to_string();
        }
    }

    "Sorry, I don't understand.".to_string()
}
