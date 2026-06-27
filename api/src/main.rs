mod server;
mod config;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "api=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _config = config::Config::from_env();

    let app = Router::new()
        .route("/", get(|| async { "SeqVault API is running" }))
        .route("/simulate", post(handlers::simulate::simulate_handler))
        .route("/commit", post(handlers::commit::commit_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
