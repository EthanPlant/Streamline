use actix_web::{App, http::StatusCode, test};
use futures::future::join_all;
use serde_json::json;
use streamline::api::init_routes;

#[actix_web::test]
async fn test_sequential_requests() {
    let app = test::init_service(App::new().configure(init_routes)).await;

    for i in 0..5 {
        let payload = json!({
            "timestamp": i,
            "service": "test-service",
            "level": "INFO",
            "message": format!("This is test log {}", i)
        });

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}

#[actix_web::test]
async fn test_concurrent_requests() {
    let app = test::init_service(App::new().configure(init_routes)).await;

    let futures = (0..5).map(|i| {
        let payload = json!({
            "timestamp": i,
            "service": "test-service",
            "level": "INFO",
            "message": format!("This is test log {}", i)
        });

        let req = test::TestRequest::post()
            .uri("/logs")
            .set_json(&payload)
            .to_request();

        test::call_service(&app, req)
    });

    let responses = join_all(futures).await;

    assert!(
        responses
            .iter()
            .all(|resp| resp.status() == StatusCode::CREATED)
    );
}
