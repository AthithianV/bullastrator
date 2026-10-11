use anyhow::Result;
use axum::http::{HeaderName, HeaderValue, Method, header};
use bullastrator_core::state::AppState;
use tower_http::cors::CorsLayer;

use crate::routes::router;

pub async fn serve(state: AppState, address: std::net::SocketAddr) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(address).await?;
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:1420"),
            HeaderValue::from_static("http://127.0.0.1:1420"),
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
            HeaderValue::from_static("http://localhost:4173"),
            HeaderValue::from_static("http://127.0.0.1:4173"),
        ])
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            HeaderName::from_static("x-requested-with"),
        ]);

    axum::serve(listener, router(state).layer(cors)).await?;
    Ok(())
}
