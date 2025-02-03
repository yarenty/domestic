use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Domestic LLM Management Utility");
    info!("Use this tool to manage models, embeddings, and system configuration");
}
