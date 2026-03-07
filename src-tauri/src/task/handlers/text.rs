use crate::task::queue::QueuedTask;
use crate::errors::Result;
use crate::ai::client::{AiClient, AiConfig, ChatRequest, ChatMessage};
use serde_json::Value;

pub async fn handle_novel_analysis(task: &QueuedTask) -> Result<String> {
    let payload = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;
    
    let content = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config = AiConfig {
        provider: "openai".to_string(),
        api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
        base_url: None,
        model: Some("gpt-3.5-turbo".to_string()),
    };

    let client = crate::ai::create_client(config)?;
    
    let system_prompt = r#"You are a professional novel analysis assistant. Analyze the given novel text and extract:
1. Main characters and their relationships
2. Key locations and settings
3. Plot structure and major events
4. Themes and motifs
5. Writing style and tone

Provide the analysis in JSON format."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Analyze this novel:\n\n{}", content),
            },
        ],
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(2000),
    };

    let response = client.chat(request).await?;
    
    Ok(response.content)
}

pub async fn handle_global_analysis(task: &QueuedTask) -> Result<String> {
    let payload = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;
    
    let content = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config = AiConfig {
        provider: "openai".to_string(),
        api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
        base_url: None,
        model: Some("gpt-3.5-turbo".to_string()),
    };

    let client = crate::ai::create_client(config)?;
    
    let system_prompt = r#"You are a content analysis assistant. Extract global information from the text:
1. All characters mentioned with brief descriptions
2. All locations/settings with descriptions
3. Key objects and items
4. Important relationships

Provide the extraction in JSON format."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Extract global information from:\n\n{}", content),
            },
        ],
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.5),
        max_tokens: Some(2000),
    };

    let response = client.chat(request).await?;
    
    Ok(response.content)
}

pub async fn handle_story_to_script(task: &QueuedTask) -> Result<String> {
    let payload = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;
    
    let content = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config = AiConfig {
        provider: "openai".to_string(),
        api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
        base_url: None,
        model: Some("gpt-3.5-turbo".to_string()),
    };

    let client = crate::ai::create_client(config)?;
    
    let system_prompt = r#"You are a professional scriptwriter. Convert the given story/novel text into a screenplay format.
Follow standard screenplay formatting:
- Scene headings (INT./EXT. LOCATION - DAY/NIGHT)
- Action descriptions
- Character names centered
- Dialogue
- Parentheticals

Maintain the story's tone, pacing, and character voices."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Convert this story to screenplay format:\n\n{}", content),
            },
        ],
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(4000),
    };

    let response = client.chat(request).await?;
    
    Ok(response.content)
}

pub async fn handle_script_to_storyboard(task: &QueuedTask) -> Result<String> {
    let payload = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;
    
    let content = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config = AiConfig {
        provider: "openai".to_string(),
        api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
        base_url: None,
        model: Some("gpt-3.5-turbo".to_string()),
    };

    let client = crate::ai::create_client(config)?;
    
    let system_prompt = r#"You are a storyboard artist and director. Convert the screenplay into a storyboard format.
For each scene/shot, provide:
1. Shot number
2. Shot type (wide, medium, close-up, etc.)
3. Camera movement (pan, tilt, zoom, etc.)
4. Visual description
5. Characters in frame
6. Duration estimate

Format as JSON array of storyboard panels."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Convert this script to storyboard:\n\n{}", content),
            },
        ],
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.6),
        max_tokens: Some(4000),
    };

    let response = client.chat(request).await?;
    
    Ok(response.content)
}
