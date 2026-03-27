//! Entry point for the Streamline service binary.
//!
//! This crate is a thin launcher that starts the Actix Web server and
//! configures the HTTP routes from the library crate.

use actix_web::{App, HttpServer};
use streamline::api;
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Launches the HTTP server and sets up logging.
///
/// The server currently binds to `127.0.0.1:8080`, but should be configurable
/// in a production-ready deployment.
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
