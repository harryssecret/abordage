#[cfg(test)]
mod tests {
    use super::super::*;
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use serde_json::json;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn check_caches_routes() {
        let db = db().await;
        let shared_state = Arc::new(AppContext { db });
        let app = app(shared_state);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/caches/new")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"cache_name": "UltimateCacheTest"}))
                            .expect("Impossible to serialize json"),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
    }
}
