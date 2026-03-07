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
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
) -> Result<Vec<Project>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_projects(pool).await {
            Ok(projects) => Ok(projects),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_project(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    name: String,
    description: Option<String>,
) -> Result<Project, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_project(pool, name, description).await {
            Ok(project) => Ok(project),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_project(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<Option<Project>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_project(pool, &id).await {
            Ok(project) => Ok(project),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_project(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<Project>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::update_project(pool, &id, name, description).await {
            Ok(project) => Ok(project),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_project(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::delete_project(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_episodes(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
) -> Result<Vec<Episode>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_episodes(pool, &project_id).await {
            Ok(episodes) => Ok(episodes),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_episode(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    content: Option<String>,
) -> Result<Episode, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_episode(pool, project_id, name, content).await {
            Ok(episode) => Ok(episode),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn import_episode(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    content: String,
) -> Result<Episode, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_episode(pool, project_id, name, Some(content)).await {
            Ok(episode) => Ok(episode),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_episode(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<Option<Episode>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_episode(pool, &id).await {
            Ok(episode) => Ok(episode),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_episode(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    content: Option<String>,
) -> Result<Option<Episode>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::update_episode(pool, &id, name, content).await {
            Ok(episode) => Ok(episode),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_episode(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::delete_episode(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_task(
    task_queue: State<Arc<TaskQueue>>,
    task_type: String,
    project: String,
    episode_id: Option<String>,
    target_type: Option<String>,
    target_id: Option<String>,
    payload: Option<Value>,
) -> Result<QueuedTask, String> {
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

    task_queue.enqueue(task.clone()).await.map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub async fn get_tasks(
    task_queue: State<Arc<TaskQueue>>,
) -> Result<Vec<QueuedTask>, String> {
    let tasks = task_queue.get_all_tasks().await;
    Ok(tasks)
}

#[tauri::command]
pub async fn get_task(
    task_queue: State<Arc<TaskQueue>>,
    task_id: String,
) -> Result<Option<QueuedTask>, String> {
    let task = task_queue.get_task(&task_id).await;
    Ok(task)
}

#[tauri::command]
pub async fn get_storyboards(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    episode_id: String,
) -> Result<Vec<Storyboard>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_storyboards(pool, &episode_id).await {
            Ok(storyboards) => Ok(storyboards),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_storyboard(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    episode_id: String,
    name: String,
) -> Result<Storyboard, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_storyboard(pool, episode_id, name).await {
            Ok(storyboard) => Ok(storyboard),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_panels(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    storyboard_id: String,
) -> Result<Vec<Panel>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_panels(pool, &storyboard_id).await {
            Ok(panels) => Ok(panels),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_panel(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    storyboard_id: String,
    panel_number: i32,
) -> Result<Panel, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_panel(pool, storyboard_id, panel_number).await {
            Ok(panel) => Ok(panel),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_panel(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
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
) -> Result<Option<Panel>, String> {
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
        match crud::update_panel(pool, &id, updates).await {
            Ok(panel) => Ok(panel),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_panel(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::delete_panel(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_global_characters(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
) -> Result<Vec<GlobalCharacter>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_global_characters(pool, &project_id).await {
            Ok(characters) => Ok(characters),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_global_character(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    description: Option<String>,
) -> Result<GlobalCharacter, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_global_character(pool, project_id, name, description).await {
            Ok(character) => Ok(character),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_global_character(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<GlobalCharacter>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::update_global_character(pool, &id, name, description).await {
            Ok(character) => Ok(character),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_global_character(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::delete_global_character(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_global_locations(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
) -> Result<Vec<GlobalLocation>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::get_global_locations(pool, &project_id).await {
            Ok(locations) => Ok(locations),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_global_location(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    description: Option<String>,
) -> Result<GlobalLocation, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::create_global_location(pool, project_id, name, description).await {
            Ok(location) => Ok(location),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_global_location(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<GlobalLocation>, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::update_global_location(pool, &id, name, description).await {
            Ok(location) => Ok(location),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_global_location(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;
    
    if let Some(pool) = pool.as_ref() {
        match crud::delete_global_location(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}
