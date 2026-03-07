use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::ai::client::{AiClient, AiConfig, ChatRequest, ChatResponse, ChatMessage};
use crate::errors::{Result, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    temperature: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Clone, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

pub struct AnthropicClient {
    config: AiConfig,
    http_client: Client,
}

impl AnthropicClient {
    pub fn new(config: AiConfig) -> Result<Self> {
        if (config.api_key.is_empty()) {
            return Err(AppError::Custom("API key is required".to_string()));
        }

        Ok(AnthropicClient {
            config,
            http_client: Client::new(),
        })
    }

    fn get_base_url(&self) -> String {
        self.config.base_url.clone().unwrap_or_else(|| {
            "https://api.anthropic.com/v1".to_string()
        })
    }

    fn get_model(&self) -> String {
        self.config.model.clone().unwrap_or_else(|| {
            "claude-3-sonnet-20240229".to_string()
        })
    }
}

#[async_trait]
impl AiClient for AnthropicClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let base_url = self.get_base_url();
        let model = self.get_model();

        let messages: Vec<AnthropicMessage> = request.messages
            .into_iter()
            .map(|m| AnthropicMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        let max_tokens = request.max_tokens.unwrap_or(4096);

        let anthropic_request = AnthropicRequest {
            model,
            messages,
            max_tokens,
            temperature: request.temperature,
        };

        let response = self.http_client
            .post(&format!("{}/messages", base_url))
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Custom(format!("Anthropic API error: {} - {}", status, error_text)));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        let content = anthropic_response
            .content
            .iter()
            .filter_map(|c| {
                if c.content_type == "text" {
                    Some(c.text.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let usage = anthropic_response.usage.map(|u| crate::ai::client::Usage {
            prompt_tokens: u.input_tokens,
            completion_tokens: u.output_tokens,
            total_tokens: u.input_tokens + u.output_tokens,
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
