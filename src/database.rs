use sqlx::{postgres::PgPoolOptions, PgPool};
use {dotenvy::dotenv, std::env};

pub async fn establish_conn() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}
