pub mod geocache;
pub mod lib;
use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/caches", geocache::router())
}
