use async_trait::async_trait;
use dotenv::dotenv;
use std::env;
use google_generative_ai_rs::v1::api::Client;
use google_generative_ai_rs::v1::gemini::{Content, Role, Part, request::Request};

use crate::core::port::AiPort;

pub struct GeminiAdapter {
    client: Client,
}

impl GeminiAdapter {
    pub fn new() -> Self {
        dotenv().ok();

        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        
        let client = Client::new(api_key);
        Self { client }
    }
}

impl Clone for GeminiAdapter {
    fn clone(&self) -> Self {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        Self {
            client: Client::new(api_key),
        }
    }
}

#[async_trait]
impl AiPort for GeminiAdapter {
    async fn send_message(&self, message: &str) -> Result<String, String> {
        let part = Part {
            text: Some(message.to_string()),
            inline_data: None,
            file_data: None,
            video_metadata: None,
        };
        let content = Content {
            role: Role::User, 
            parts: vec![part],
        };
        let request = Request {
            contents: vec![content],
            tools: vec![], 
            safety_settings: vec![],
            generation_config: None,
        };

        match self.client.post(30, &request).await {
            Ok(post_result) => match post_result.rest() {
                Some(response) => {
                    if let Some(candidate) = response.candidates.get(0) {
                        let output_parts: Vec<String> = candidate
                            .content
                            .parts
                            .iter()
                            .filter_map(|part| part.text.clone())
                            .collect();

                        Ok(output_parts.join(" ")) 
                    } else {
                        Err("No candidates received".to_string())
                    }
                },
                None => Err("No REST response received".to_string()),
            },
            Err(err) => Err(format!("API request failed: {}", err)),
        }
    }
}
