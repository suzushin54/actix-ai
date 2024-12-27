use crate::core::entities::{ChatRequest, ChatResponse};
use crate::core::port::AiPort;
use async_trait::async_trait;

pub async fn ai_interface<P: AiPort>(req: ChatRequest, ai_port: &P) -> ChatResponse { 
    match ai_port.send_message(&req.message).await {
        Ok(response) => ChatResponse { response },
        Err(err) => ChatResponse {
            response: format!("Error: {}", err),
        },
    }
}
