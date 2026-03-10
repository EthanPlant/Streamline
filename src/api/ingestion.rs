use actix_web::{HttpResponse, Responder, post, web::Json};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LogPlayload {
    pub timestamp: i64,
    pub service: String,
    pub level: String,
    pub message: String,
}

#[post("/logs")]
async fn ingest_log(log: Json<LogPlayload>) -> impl Responder {
    println!(
        "Recieved log: [{}], {} - {}",
        log.level, log.service, log.message
    );
    HttpResponse::Ok().body("Log received")
}
