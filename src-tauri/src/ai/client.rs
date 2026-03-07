use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub api_key: String,
    pub base_url: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[async_trait]
pub trait AiClient: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    
    async fn chat_stream(&self, request: ChatRequest) -> Result<tokio::sync::mpsc::Receiver<String>>;
    
    fn get_config(&self) -> &AiConfig;
}

pub fn create_client(config: AiConfig) -> Result<Box<dyn AiClient>> {
    match config.provider.as_str() {
        "openai" => Ok(Box::new(crate::ai::providers::openai::OpenAiClient::new(config)?)),
        "anthropic" => Ok(Box::new(crate::ai::providers::anthropic::AnthropicClient::new(config)?)),
        _ => Err(crate::errors::AppError::Custom(format!("Unknown AI provider: {}", config.provider))),
    }
}
