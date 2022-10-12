#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sqlx::{Connection, PgConnection};
    #[test]
    fn add_caches_to_db() {
        dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("Impossible to parse DATABASE_URL.");
        let conn = PgConnection::connect(&database_url);
    }

    #[test]
    fn check_routes() {}
}
