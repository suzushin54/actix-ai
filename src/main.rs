mod core;
mod adapters;

use actix_web::{web, App, HttpServer};
use adapters::gemini::GeminiAdapter;
use adapters::controllers::handle_chat;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gemini_adapter = GeminiAdapter::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(gemini_adapter.clone()))
            .route("/chat", web::post().to(handle_chat))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
