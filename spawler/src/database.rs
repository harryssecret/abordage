use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Connection, Pool, Postgres};
use std::env;

pub async fn etablish_connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Impossible to get environment variable.");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?
}
