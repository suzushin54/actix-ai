use actix_web::{web, Responder, HttpResponse};
use crate::core::usecase::ai_interface;
use crate::core::entities::ChatRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HttpChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct HttpChatResponse {
    pub response: String,
}

pub async fn chat(req: web::Json<HttpChatRequest>) -> impl Responder {
    let chat_request = ChatRequest {
        message: req.message.clone(),
    };

    let chat_response = ai_interface(chat_request);

    HttpResponse::Ok().json(HttpChatResponse {
        response: chat_response.response,
    })
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("running!")
}
