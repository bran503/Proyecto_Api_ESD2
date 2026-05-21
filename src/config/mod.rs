use sqlx::PgPool;

pub async fn init_db_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let url = dotenvy::var("DATABASE_URL").expect("Falta DATABASE_URL en .env");
    PgPool::connect(&url).await.expect("Error conectando a PostgreSQL")
}
