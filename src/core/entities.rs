use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
}
