use anyhow::Result;
use bullastrator_web::server::{serve, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://bullastrator.db".into());
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let pool = sqlx::SqlitePool::connect(&database_url).await?;
    let state = AppState::new(pool, &redis_url, "default", "bull")?;
    serve(state, ([127, 0, 0, 1], 3000).into()).await
}
