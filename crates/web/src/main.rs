use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .route("/", get(health_check))
        .layer(TraceLayer::new_for_http());

    info!("Starting web server on 0.0.0.0:3000");
    
    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Domestic LLM Web Interface is running!"
}
