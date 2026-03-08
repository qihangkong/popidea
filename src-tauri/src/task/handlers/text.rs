use crate::task::queue::QueuedTask;
use crate::errors::Result;
use crate::ai::client::{AiClient, AiConfig, ChatRequest, ChatMessage};
use serde_json::Value;

// ==================== Novel Analysis ====================

pub async fn handle_novel_analysis(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let content: &str = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a professional novel analysis assistant. Analyze given novel text and extract:
1. Main characters and their relationships
2. Key locations and settings
3. Plot structure and major events
4. Themes and motifs
5. Writing style and tone

Provide analysis in JSON format with fields: characters, locations, plot_structure, themes, writing_style."#;

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
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(2000),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

pub async fn handle_global_analysis(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let content: &str = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a content analysis assistant. Extract global information from text:
1. All characters mentioned with brief descriptions
2. All locations/settings with descriptions
3. Key objects and items
4. Important relationships

Provide extraction in JSON format with fields: characters, locations, objects, relationships."#;

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
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.5),
        max_tokens: Some(2000),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

// ==================== Script Conversion ====================

pub async fn handle_story_to_script(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let content: &str = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a professional scriptwriter. Convert given story/novel text into a screenplay format.
Follow standard screenplay formatting:
- Scene headings (INT./EXT. LOCATION - DAY/NIGHT)
- Action: descriptions of what happens in scene
- Character names centered (ALL CAPS)
- Dialogue: character speech
- Parentheticals: (directions for how to speak lines)

Maintain story's tone, pacing, and character voices."#;

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
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(4000),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

pub async fn handle_script_to_storyboard(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let content: &str = payload.get
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No content in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a storyboard artist and director. Convert screenplay into a storyboard format.
For each scene/shot, provide:
1. shot_number: sequential number
2. shot_type: (wide, medium, close-up, extreme close-up, over-the-shoulder, etc.)
3. camera_move: (static, pan, tilt, zoom, dolly, tracking, etc.)
4. description: visual description of what's in frame
5. characters: list of character names visible in frame
6. location: where this shot takes place
7. duration: estimated duration in seconds
8. audio: dialogue or sound notes

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
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.6),
        max_tokens: Some(4000),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

// ==================== Asset Design ====================

pub async fn handle_ai_design_character(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let user_instruction: &str = payload.get("userInstruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No userInstruction in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a character designer for visual media. Based on user's description, create a detailed prompt for generating a character image.

Analyze user's request and create a comprehensive image generation prompt that includes:
1. Physical appearance details (age, gender, body type, facial features)
2. Clothing and accessories
3. Stylistic elements (art style, color palette, lighting mood)
4. Composition and framing suggestions

Return a JSON object with a single field "prompt" containing detailed image generation prompt."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Design a character based on this description:\n{}", user_instruction),
            },
        ],
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(1200),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

pub async fn handle_ai_design_location(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let user_instruction: &str = payload.get("userInstruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No userInstruction in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a location/set designer for visual media. Based on user's description, create a detailed prompt for generating a location/environment image.

Analyze user's request and create a comprehensive image generation prompt that includes:
1. Environmental details (indoor/outdoor, time of day, weather)
2. Architectural elements (buildings, furniture, props)
3. Atmosphere and mood
4. Stylistic elements (art style, color palette, lighting)
5. Composition and framing suggestions

Return a JSON object with a single field "prompt" containing detailed image generation prompt."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Design a location based on this description:\n{}", user_instruction),
            },
        ],
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(1200),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

pub() fn handle_ai_modify_appearance(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let current_description: &str = payload.get("currentDescription")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No currentDescription in payload".to_string()))?;

    let modification_instruction: &str = payload.get("modificationInstruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No modificationInstruction in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a character appearance editor. Modify given character description based on user's instructions.

Take current character appearance and apply requested modifications while maintaining:
1. Consistency with original design
2. Character identity and personality
3. Visual coherence

Return a JSON object with a single field "prompt" containing modified image generation prompt."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!(
                    "Current appearance:\n{}\n\nModify it with these instructions:\n{}",
                    current_description, modification_instruction
                ),
            },
        ],
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(1200),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

pub async fn handle_ai_modify_location(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let current_description: &str = payload.get("currentDescription")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No currentDescription in payload".to_string()))?;

    let modification_instruction: &str = payload.get("modificationInstruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No modificationInstruction in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a location/set designer. Modify given location description based on user's instructions.

Take current location design and apply requested modifications while maintaining:
1. Consistency with original design
2. Location function and atmosphere
3. Visual coherence

Return a JSON object with a single field "prompt" containing modified image generation prompt."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!(
                    "Current location:\n{}\n\nModify it with these instructions:\n{}",
                    current_description, modification_instruction
                ),
            },
        ],
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(1200),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

// ==================== Shot Prompt Modification ====================

pub async fn handle_ai_modify_shot_prompt(task: &QueuedTask) -> Result<String> {
    let payload: &Value = task.payload.as_ref()
        .ok_or_else(|| crate::errors::AppError::Custom("No payload provided".to_string()))?;

    let current_prompt: &str = payload.get("currentPrompt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No currentPrompt in payload".to_string()))?;

    let modification_instruction: &str = payload.get("modificationInstruction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::Custom("No modificationInstruction in payload".to_string()))?;

    let config: AiConfig = get_ai_config_from_payload(payload)?;

    let client: AiClient = crate::ai::create_client(config)?;

    let system_prompt: &str = r#"You are a storyboard shot editor. Modify given shot prompt based on user's instructions.

Take current storyboard shot prompt and apply requested modifications while maintaining:
1. Shot type and camera movement consistency
2. Story context and continuity
3. Visual clarity for image/video generation

Return a JSON object with a single field "prompt" containing modified shot prompt."#;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!(
                    "Current shot prompt:\n{}\n\nModify it with these instructions:\n{}",
                    current_prompt, modification_instruction
                ),
            },
        ],
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(1200),
    };

    let response = client.chat(request).await?;

    Ok(response.content)
}

// ==================== Helper Functions ====================

fn get_ai_config_from_payload(payload: &Value) -> AiConfig {
    let provider: &str = payload.get("provider")
        .and_then(|v| v.as_str())
        .unwrap_or("openai");

    let api_key: String = payload.get("apiKey")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string())
        });

    let base_url: Option<String> = payload.get("baseUrl")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let model: String = payload.get("model")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "gpt-4o-mini".to_string());

    AiConfig {
        provider: provider.to_string(),
        api_key,
        base_url,
        model: Some(model),
    }
}
