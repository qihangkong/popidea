use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use crate::db::crud;
use crate::db::models::*;
use crate::task::queue::{TaskQueue, QueuedTask, TaskStatus};
use crate::errors::{Result, AppError};
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn get_projects(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
) -> Result<Vec<Project>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_projects(pool).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_project(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    name: String,
    description: Option<String>,
) -> Result<Project> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_project(pool, name, description).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn get_project(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<Option<Project>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_project(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn update_project(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<Project>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::update_project(pool, &id, name, description).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn delete_project(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::delete_project(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn get_episodes(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
) -> Result<Vec<Episode>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_episodes(pool, &project_id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_episode(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    content: Option<String>,
) -> Result<Episode> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_episode(pool, project_id, name, content).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn import_episode(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    content: String,
) -> Result<Episode> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_episode(pool, project_id, name, Some(content)).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn get_episode(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<Option<Episode>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_episode(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn update_episode(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    content: Option<String>,
) -> Result<Option<Episode>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::update_episode(pool, &id, name, content).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn delete_episode(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::delete_episode(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_task(
    task_queue: State<'_, Arc<TaskQueue>>,
    task_type: String,
    project: String,
    episode_id: Option<String>,
    target_type: Option<String>,
    target_id: Option<String>,
    payload: Option<Value>,
) -> Result<QueuedTask> {
    let task = QueuedTask {
        id: Uuid::new_v4().to_string(),
        task_type,
        status: TaskStatus::Queued,
        project,
        episode_id,
        target_type,
        target_id,
        payload,
        result: None,
        progress: 0,
        error_message: None,
        created_at: Utc::now().timestamp(),
        started_at: None,
        finished_at: None,
    };

    task_queue.enqueue(task.clone()).await?;
    Ok(task)
}

#[tauri::command]
pub async fn get_tasks(
    task_queue: State<'_, Arc<TaskQueue>>,
) -> Result<Vec<QueuedTask>> {
    let tasks = task_queue.get_all_tasks().await;
    Ok(tasks)
}

#[tauri::command]
pub async fn get_task(
    task_queue: State<'_, Arc<TaskQueue>>,
    task_id: String,
) -> Result<Option<QueuedTask>> {
    let task = task_queue.get_task(&task_id).await;
    Ok(task)
}

#[tauri::command]
pub async fn get_storyboards(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    episode_id: String,
) -> Result<Vec<Storyboard>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_storyboards(pool, &episode_id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_storyboard(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    episode_id: String,
    name: String,
) -> Result<Storyboard> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_storyboard(pool, episode_id, name).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn get_panels(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    storyboard_id: String,
) -> Result<Vec<Panel>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_panels(pool, &storyboard_id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_panel(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    storyboard_id: String,
    panel_number: i32,
) -> Result<Panel> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_panel(pool, storyboard_id, panel_number).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn update_panel(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    shot_type: Option<String>,
    camera_move: Option<String>,
    description: Option<String>,
    location: Option<String>,
    characters: Option<String>,
    srt_start: Option<f64>,
    srt_end: Option<f64>,
    duration: Option<f64>,
    video_prompt: Option<String>,
    image_url: Option<String>,
    video_url: Option<String>,
) -> Result<Option<Panel>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        let updates = crud::PanelUpdate {
            shot_type,
            camera_move,
            description,
            location,
            characters,
            srt_start,
            srt_end,
            duration,
            video_prompt,
            image_url,
            video_url,
        };
        crud::update_panel(pool, &id, updates).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn delete_panel(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::delete_panel(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn get_global_characters(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
) -> Result<Vec<GlobalCharacter>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_global_characters(pool, &project_id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_global_character(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    description: Option<String>,
) -> Result<GlobalCharacter> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_global_character(pool, project_id, name, description).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn update_global_character(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<GlobalCharacter>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::update_global_character(pool, &id, name, description).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn delete_global_character(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::delete_global_character(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn get_global_locations(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
) -> Result<Vec<GlobalLocation>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::get_global_locations(pool, &project_id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn create_global_location(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    description: Option<String>,
) -> Result<GlobalLocation> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::create_global_location(pool, project_id, name, description).await

 } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn update_global_location(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<GlobalLocation>> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::update_global_location(pool, &id, name, description).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}

#[tauri::command]
pub async fn delete_global_location(
    db_pool: State<'_, Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        crud::delete_global_location(pool, &id).await
    } else {
        Err(AppError::Custom("Database pool not initialized".to_string()))
    }
}