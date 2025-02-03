use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub model: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub model: String,
}

pub async fn chat(req: Request<ChatRequest>) -> Result<Response> {
    let chat_request = req.payload()?;
    let model = chat_request.model.unwrap_or_else(|| "default".to_string());
    
    // TODO: Implement actual chat logic here
    let response = ChatResponse {
        response: format!("Echo: {}", chat_request.message),
        model,
    };
    
    Ok(response.into())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api")
        .add("/chat", post(chat))
}
