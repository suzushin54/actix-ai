use actix_web::{web, App, HttpServer};
mod adapters;
mod core;

use adapters::controllers::{chat, status};

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
