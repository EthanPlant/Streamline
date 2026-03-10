use actix_web::{HttpResponse, Responder, post, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogPlayload {
    pub timestamp: i64,
    pub service: String,
    pub level: String,
    pub message: String,
}

impl LogPlayload {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.service.trim().is_empty() {
            return Err("Service cannot be empty");
        }
        if self.level.trim().is_empty() {
            return Err("Level cannot be empty");
        }
        if self.message.trim().is_empty() {
            return Err("Message cannot be empty");
        }

        Ok(())
    }
}

#[post("/logs")]
async fn ingest_log(log: Json<LogPlayload>) -> impl Responder {
    match log.validate() {
        Ok(_) => {
            println!(
                "Received log: [{}], {} - {}",
                log.level, log.service, log.message
            );
            HttpResponse::Created().body("Log received")
        }
        Err(e) => {
            println!("Validation error: {}", e);
            HttpResponse::BadRequest().body(format!("Invalid log payload: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test};

    use crate::api::init_routes;

    use super::*;

    #[actix_web::test]
    async fn test_valid_paylod() {
        let payload = LogPlayload {
            timestamp: 0,
            service: "test-service".into(),
            level: "INFO".into(),
            message: "This is a test log".into(),
        };
        assert!(payload.validate().is_ok());
    }

    #[actix_web::test]
    async fn test_empty_service() {
        let payload = LogPlayload {
            timestamp: 0,
            service: "".into(),
            level: "INFO".into(),
            message: "This is a test log".into(),
        };
        assert!(payload.validate().is_err());
    }

    #[actix_web::test]
    async fn test_empty_level() {
        let payload = LogPlayload {
            timestamp: 0,
            service: "test-service".into(),
            level: "".into(),
            message: "This is a test log".into(),
        };
        assert!(payload.validate().is_err());
    }

    #[actix_web::test]
    async fn test_empty_message() {
        let payload = LogPlayload {
            timestamp: 0,
            service: "test-service".into(),
            level: "INFO".into(),
            message: "".into(),
        };
        assert!(payload.validate().is_err());
    }

    #[actix_web::test]
    async fn test_endpoint_valid_payload() {
        let app = test::init_service(App::new().configure(init_routes)).await;

        let payload = LogPlayload {
            timestamp: 0,
            service: "test-service".into(),
            level: "INFO".into(),
            message: "This is a test log".into(),
        };

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_endpoint_invalid_payload() {
        let app = test::init_service(App::new().configure(init_routes)).await;

        let payload = LogPlayload {
            timestamp: 0,
            service: "".into(),
            level: "INFO".into(),
            message: "This is a test log".into(),
        };

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
