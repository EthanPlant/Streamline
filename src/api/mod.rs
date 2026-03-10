use actix_web::web;

use crate::api::ingestion::ingest_log;

mod ingestion;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ingest_log);
}
