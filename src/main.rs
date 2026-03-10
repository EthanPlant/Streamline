use actix_web::{App, HttpServer};
use tracing::info;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server on http://127.0.0.1:8080");
    HttpServer::new(|| App::new().configure(api::init_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
