//! API routing and endpoint registration for Streamline.
//!
//! This module wires together individual endpoint handlers and exposes the
//! route configuration function for the server bootstrap.

use actix_web::web;

use crate::api::ingestion::ingest_log;

mod ingestion;

/// Add API service routes to Actix Web.
///
/// # Example
///
/// ```rust
/// App::new().configure(streamline::api::init_routes);
/// ```
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ingest_log);
}
