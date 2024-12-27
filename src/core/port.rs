pub trait AiPort {
    fn send_message(&self, message: &str) -> Result<String, String>;
}
