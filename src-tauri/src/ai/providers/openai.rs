use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::ai::client::{AiClient, AiConfig, ChatRequest, ChatResponse, ChatMessage};
use crate::errors::{Result, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
    usage: Option<OpenAiUsage>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

pub struct OpenAiClient {
    config: AiConfig,
    http_client: Client,
}

impl OpenAiClient {
    pub fn new(config: AiConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(AppError::Custom("API key is required".to_string()));
        }

        Ok(OpenAiClient {
            config,
            http_client: Client::new(),
        })
    }

    fn get_base_url(&self) -> String {
        self.config.base_url.clone().unwrap_or_else(|| {
            "https://api.openai.com/v1".to_string()
        })
    }

    fn get_model(&self) -> String {
        self.config.model.clone().unwrap_or_else(|| {
            "gpt-3.5-turbo".to_string()
        })
    }
}

#[async_trait]
impl AiClient for OpenAiClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let base_url = self.get_base_url();
        let model = self.get_model();

        let messages: Vec<OpenAiMessage> = request.messages
            .into_iter()
            .map(|m| OpenAiMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        let openai_request = OpenAiRequest {
            model,
            messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let response = self.http_client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Custom(format!("OpenAI API error: {} - {}", status, error_text)));
        }

        let openai_response: OpenAiResponse = response.json().await?;

        let content = openai_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| String::new());

        let usage = openai_response.usage.map(|u| crate::ai::client::Usage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok(ChatResponse { content, usage })
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<tokio::sync::mpsc::Receiver<String>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        let response = self.chat(_request).await?;
        let _ = tx.send(response.content).await;
        
        Ok(rx)
    }

    fn get_config(&self) -> &AiConfig {
        &self.config
    }
}
