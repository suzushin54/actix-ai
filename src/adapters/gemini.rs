use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::core::port::AiPort;
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct GeminiAdapter {
    api_key: String,
    client: Client,
    endpoint: String,
}

impl GeminiAdapter {
    pub fn new() -> Self {
        dotenv().ok();

        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        let project_id = env::var("PROJECT_ID").expect("PROJECT_ID must be set");
        let location = env::var("LOCATION").expect("LOCATION must be set");
        let model_id = env::var("MODEL_ID").expect("MODEL_ID must be set");

        Self {
            api_key,
            client: Client::new(),
            endpoint: format!(
                "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
                location, project_id, location, model_id
            ),
        }
    }
}

#[derive(Serialize)]
struct GenerateContentRequest<'a> {
    contents: Vec<&'a str>,
}

#[derive(Deserialize)]
struct GenerateContentResponse {
}

#[async_trait]
impl AiPort for GeminiAdapter {
    async fn send_message(&self, message: &str) -> Result<String, String> {
        let request_body = GenerateContentRequest {
            contents: vec![message],
        };

        let response = self
        .client
        .post(&self.endpoint)
        .bearer_auth(&self.api_key)
        .json(&request_body)
        .send()
        .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<GenerateContentResponse>().await {
                    Ok(parsed) => {
                        Ok("Success".to_string())
                    }
                    Err(_) => Err("Failed to parse response".to_string()),
                }
            }
            Ok(resp) => Err(format!("API Error: {}", resp.status())),
            Err(err) => Err(format!("Request Error: {}", err)),
        }
    }
}
