use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health_check", get(health_check_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    tracing::debug!("Listening on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check_handler() -> StatusCode {
    tracing::info!("Health check request");
    StatusCode::OK
}
