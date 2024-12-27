mod adapters;
mod core;

use adapters::gemini::GeminiAdapter;
use core::usecase::ai_interface;
use core::entities::{ChatRequest};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gemini_adapter = GeminiAdapter::new("API_KEY".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(gemini_adapter.clone()))
            .route("/chat", web::post().to(chat_handler))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn chat_handler(
    chat_req: web::Json<ChatRequest>, 
    gemini_adapter: web::Data<GeminiAdapter>,
) -> impl Responder {
    let response = ai_interface(chat_req.into_inner(),
    &*gemini_adapter).await;
    HttpResponse::Ok().json(response)
}
