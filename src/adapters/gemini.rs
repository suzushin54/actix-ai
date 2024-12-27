use reqwest::Client;
use async_trait::async_trait;
use crate::core::port::AiPort;

pub struct GeminiAdapter {
    api_key: String,
    client: Client,
    base_url: String,
}

impl GeminiAdapter {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://api.gemini.com/v1/chat".to_string(),
        }
    }
}

#[async_trait]
impl AiPort for GeminiAdapter {
    async fn send_message(&self, messsag: &str) -> Result<String, String> {
        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({"message": message }))
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) =>
                    Ok(json["response"].as_str().unwrap_or_default().to_string()),
                    Err(_) => Err("Failed to parse response".to_string()),
                }
            }
            Ok(resp) => Err(format!("API Error: {}", resp.status())),
            Err(err) => Err(format!("Response Error: {}", err)),
        }
    }
}
