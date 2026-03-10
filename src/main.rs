use actix_web::{App, HttpServer};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Starting Streamline API server on 127.0.0.1:8080");

    HttpServer::new(|| App::new().configure(api::init_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
