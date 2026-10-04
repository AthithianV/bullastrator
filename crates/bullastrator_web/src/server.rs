use anyhow::Result;
use bullastrator_core::state::AppState;

use crate::routes::router;

pub async fn serve(state: AppState, address: std::net::SocketAddr) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, router(state)).await?;
    Ok(())
}
