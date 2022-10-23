#[cfg(test)]
mod tests {
    use super::super::*;
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn check_users_routes() {
        let db = db().await;
        let shared_state = Arc::new(AppContext { db });
        let app = app(shared_state);

        let users_list = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(users_list.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn check_users_creation() {
        let db = db().await;
        let shared_state = Arc::new(AppContext { db });
        let app = app(shared_state);

        let add_user_req = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/users/new")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "username": "John Doe",
                            "password": "password",
                        }))
                        .expect("failed to serialize json"),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(add_user_req.status(), http::StatusCode::CREATED);
    }

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
                        serde_json::to_vec(&json!({"cache_name": "UltimateCacheTest", "maintainer": "John Doe", "location": {"type": "Point", "coordinates": [-1, -2]}, "difficulty": 1})).unwrap())).unwrap()).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::CREATED);
    }
}
