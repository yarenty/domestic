use eyre::Result;
use loco_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let _ctx = Application::new()?
        .configure(|app| {
            app.with_name("domestic-web")
                .with_version("0.3.0")
        })
        .initialize()
        .await?;

    info!("🚀 Domestic LLM Web Interface is running!");
    
    Ok(())
}

// async fn health_check() -> &'static str {
//     "Domestic LLM Web Interface is running!"
// }
