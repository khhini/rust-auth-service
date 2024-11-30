use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::DatabaseConfig;

pub struct PostgresDatabase {
    pub pool: PgPool
}

impl PostgresDatabase {
   pub async fn new(config: DatabaseConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.get_connection_string())
            .await
            .unwrap();

        Self { pool }
    }
}
