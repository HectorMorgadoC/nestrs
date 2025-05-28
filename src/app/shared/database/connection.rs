pub mod connection {
    use sqlx::{postgres::PgPoolOptions, PgPool};

    pub async fn init_pool(databse_url: &str) -> Result<PgPool,sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(20)
            .connect(databse_url)
            .await
    } 

}