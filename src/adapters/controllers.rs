use actix_web::{web, HttpResponse, Responder};
use crate::core::entities::ChatRequest;
use crate::core::usecase::ai_interface;
use crate::adapters::gemini::GeminiAdapter;

pub async fn handle_chat(
    chat_req: web::Json<ChatRequest>,
    gemini_adapter: web::Data<GeminiAdapter>,
) -> impl Responder {
    let response = ai_interface(chat_req.into_inner(), &*gemini_adapter).await;
    HttpResponse::Ok().json(response)
}
