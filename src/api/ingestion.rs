use actix_web::{HttpResponse, ResponseError, post, web::Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogPlayload {
    pub timestamp: i64,
    pub service: String,
    pub level: String,
    pub message: String,
}

impl LogPlayload {
    pub fn validate(&self) -> Result<(), IngestionError> {
        if self.service.trim().is_empty() {
            return Err(IngestionError::ValidationError(
                "Service cannot be empty".into(),
            ));
        }
        if self.level.trim().is_empty() {
            return Err(IngestionError::ValidationError(
                "Level cannot be empty".into(),
            ));
        }
        if self.message.trim().is_empty() {
            return Err(IngestionError::ValidationError(
                "Message cannot be empty".into(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ResponseError for IngestionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IngestionError::ValidationError(msg) => {
                HttpResponse::BadRequest().body(msg.to_string())
            }
        }
    }
}

#[post("/logs")]
async fn ingest_log(payload: Json<LogPlayload>) -> Result<HttpResponse, IngestionError> {
    let log = payload.into_inner();
    log.validate()?;
    println!("Received log: {log:?}");
    Ok(HttpResponse::Created().body("Log received"))
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

    #[actix_web::test]
    async fn test_endpoint_missing_field() {
        let app = test::init_service(App::new().configure(init_routes)).await;

        let payload = serde_json::json!({
            "timestamp": 0,
            "service": "test-service",
            "level": "INFO"
        });

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_endpoint_wrong_type() {
        let app = test::init_service(App::new().configure(init_routes)).await;

        let payload = serde_json::json!({
            "timestamp": "not-a-timestamp",
            "service": "test-service",
            "level": "INFO",
            "message": "This is a test log"
        });

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_endpoint_extra_field() {
        let app = test::init_service(App::new().configure(init_routes)).await;

        let payload = serde_json::json!({
            "timestamp": 0,
            "service": "test-service",
            "level": "INFO",
            "message": "This is a test log",
            "extra_field": "extra_value"
        });

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
