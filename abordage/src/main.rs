use axum::{routing::get, Extension, Router};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::{env, sync::Arc};
use tower::ServiceBuilder;

pub mod caches;
mod tests;
pub mod users;

#[tokio::main]
async fn main() {
    let db = db().await;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Migrations failed to run.");

    let shared_state = Arc::new(AppContext { db });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(shared_state).into_make_service())
        .await
        .unwrap();
}

pub async fn db() -> Pool<Postgres> {
    dotenvy::dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").expect("Impossible to get DATABASE_URL environment variable");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Impossible to connect to the database.")
}

pub fn app(shared_state: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", get(|| async { "abordage is running:)" }))
        .merge(caches::router())
        .merge(users::router())
        .layer(ServiceBuilder::new().layer(Extension(shared_state)))
}

pub struct AppContext {
    db: PgPool,
}
