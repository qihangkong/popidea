use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use crate::task::queue::{TaskQueue, QueuedTask, TaskStatus};
use crate::errors::{Result, AppError};
use uuid::Uuid;
use chrono::Utc;

// ==================== Novel Analysis ====================

#[tauri::command]
pub async fn analyze_novel(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    episode_id: Option<String>,
    content: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "novel_analysis".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id,
        target_type: Some("episode".to_string()),
        target_id: None,
        payload: Some(serde_json::json!({
            "content": content,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

// ==================== Global Analysis ====================

#[tauri::command]
pub async fn analyze_global(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    episode_id: Option<String>,
    content: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "global_analysis".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id,
        target_type: Some("episode".to_string()),
        target_id: None,
        payload: Some(serde_json::json!({
            "content": content,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

// ==================== Story to Script Script ====================

#[tauri::command]
pub async fn convert_story_to_script(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    episode_id: Option<String>,
    content: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "story_to_script".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id,
        target_type: Some("episode".to_string()),
        target_id: None,
        payload: Some(serde_json::json!({
            "content": content,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

// ==================== Script to Storyboard Script ====================

#[tauri::command]
pub async fn convert_script_to_storyboard(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    episode_id: Option<String>,
    content: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "script_to_storyboard".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id,
        target_type: Some("episode".to_string()),
        target_id: None,
        payload: Some(serde_json::json!({
            "content": content,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

// ==================== Asset Design Script ====================

#[tauri::command]
pub async fn ai_design_character(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    user_instruction: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "ai_design_character".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id: None,
        target_type: Some("character".to_string()),
        target_id: None,
        payload: Some(serde_json::json!({
            "userInstruction": user_instruction,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub async fn ai_design_location(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    user_instruction: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "ai_design_location".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id: None,
        target_type: Some("location".to_string()),
        target_id: None,
        payload: Some(serde_json::json!({
            "userInstruction": user_instruction,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

// ==================== Asset Modification Script ====================

#[tauri::command]
pub async fn ai_modify_appearance(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    character_id: String,
    current_description: String,
    modification_instruction: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "ai_modify_appearance".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id: None,
        target_type: Some("character_appearance".to_string()),
        target_id: Some(character_id),
        payload: Some(serde_json::json!({
            "currentDescription": current_description,
            "modificationInstruction": modification_instruction,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub async fn ai_modify_location(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    location_id: String,
    current_description: String,
    modification_instruction: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "ai_modify_location".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id: None,
        target_type: Some("location".to_string()),
        target_id: Some(location_id),
        payload: Some(serde_json::json!({
            "currentDescription": current_description,
            "modificationInstruction": modification_instruction,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

// ==================== Shot Prompt Modification Script ====================

#[tauri::command]
pub async fn ai_modify_shot_prompt(
    task_queue: State<Arc<TaskQueue>>,
    project_id: String,
    panel_id: String,
    current_prompt: String,
    modification_instruction: String,
    provider: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<QueuedTask, String> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type: "ai_modify_shot_prompt".to_string(),
        status: TaskStatus::Queued,
        project: project_id,
        episode_id: None,
        target_type: Some("panel".to_string()),
        target_id: Some(panel_id),
        payload: Some(serde_json::json!({
            "currentPrompt": current_prompt,
            "modificationInstruction": modification_instruction,
            "provider": provider.unwrap_or_else(|| "openai".to_string()),
            "apiKey": api_key,
            "model": model,
        })),
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}
