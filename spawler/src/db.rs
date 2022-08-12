use dotenvy::dotenv;
use sqlx::{database, postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn etablish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Impossible to get environment variable.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
