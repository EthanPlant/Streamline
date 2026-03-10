use actix_web::{App, HttpServer, Responder, web};
use tracing::info;

async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server on http://127.0.0.1:8080");
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
