use crate::core::entities::{ChatRequest, ChatResponse};
use std::collections::HashMap;

pub fn ai_interface(req: ChatRequest) -> ChatResponse { 
    let mut ress = HashMap::new();
    ress.insert("Hello", "Hi! What's up?");
    ress.insert("Good bye", "See you tomorrow!");

    let res = ress
    .iter()
    .find(|(key, _)| req.message.contains(*key))
    .map(|(_, value)| value.to_string())
    .unwrap_or_else(|| "Sorry, I don't understand.".to_string());

    ChatResponse {
        response: res,
    }
}
