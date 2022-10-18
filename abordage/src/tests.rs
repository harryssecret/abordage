#[cfg(test)]
mod tests {
    use super::super::*;
    use axum::{body::Body, http::Request};
    use tower::{Service, ServiceExt};
    #[tokio::test]
    async fn check_caches_routes() {
        let db = db().await;
        let shared_state = Arc::new(AppContext { db });
        let app = app(shared_state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/caches/new")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
    }
}
