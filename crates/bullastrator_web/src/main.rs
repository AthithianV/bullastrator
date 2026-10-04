use anyhow::Result;
use bullastrator_core::state::AppState;
use bullastrator_web::server::serve;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let pool = bullastrator_storage::initialize_database().await?;
    let state = AppState::new(pool).await?;
    serve(state, ([127, 0, 0, 1], 3000).into()).await
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new("bullastrator_core=debug,bullastrator_web=debug")
    });
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .init();
}
