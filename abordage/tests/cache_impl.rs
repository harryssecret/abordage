#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test(migrations = "./migrations")]
    async fn add_caches_to_db(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        Ok(())
    }

    #[test]
    fn check_routes() {}
}
