use actix_web::{web, App, HttpServer, Responder, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/status", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn status() -> impl Responder {
    HttpResponse::Ok().body("running!")
}
