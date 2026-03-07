use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
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
    payload: Option<String>,
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
