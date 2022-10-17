use axum::{routing::get, Extension, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub mod caches;
pub mod users;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").expect("Impossible to get DATBASE_URL environment variable");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Impossible to connect to the database.");

    let app = Router::new()
        .route("/", get(|| async { "abordage is running:)" }))
        .merge(caches::router());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct AppContext {
    db: PgPool,
}
