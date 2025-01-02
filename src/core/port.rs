use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait AiPort: Send + Sync {
    async fn send_message(&self, message: &str) -> Result<String, String>;
}

#[async_trait]
impl<T: AiPort + ?Sized> AiPort for Arc<T> {
    async fn send_message(&self, message: &str) -> Result<String, String> {
        (**self).send_message(message).await
    }
}
