use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;

pub mod types;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").expect("Impossible to get DATBASE_URL environment variable");

    let db = PgPoolOptions::new().max_connections(5).connect(&db_url);

    let app = Router::new().route("/", get(|| async { "hii" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
