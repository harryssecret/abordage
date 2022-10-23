use axum::Router;

pub mod users;

pub fn router() -> Router {
    Router::new().nest("/users", users::router())
}
