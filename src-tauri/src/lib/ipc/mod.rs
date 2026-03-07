use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

use crate::lib::{
    db::{crud, Database},
    storage::StorageService,
    task::TaskQueue,
};

pub struct IpcHandler {
    db: Arc<Mutex<Database>>,
    storage: Arc<Mutex<StorageService>>,
    task_queue: Arc<Mutex<TaskQueue>>,
}

impl IpcHandler {
    pub fn new(
        db: Arc<Mutex<Database>>,
        storage: Arc<Mutex<StorageService>>,
        task_queue: Arc<Mutex<TaskQueue>>,
    ) -> Self {
        Self {
            db,
            storage,
            task_queue,
        }
    }

    pub fn register(self, app_handle: &AppHandle) {
        app_handle.manage(self.db.clone());
        app_handle.manage(self.storage.clone());
        app_handle.manage(self.task_queue.clone());
    }
}

#[tauri::command]
pub async fn list_projects(
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<crate::lib::db::models::Project>, String> {
    let db = db.lock().await;
    crud::ProjectRepository::list(db.pool())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_project(
    db: State<'_, Arc<Mutex<Database>>>,
    name: String,
    description: Option<String>,
) -> Result<crate::lib::db::models::Project, String> {
    let db = db.lock().await;
    crud::ProjectRepository::create(db.pool(), name, description)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_project(
    db: State<'_, Arc<Mutex<Database>>>,
    id: String,
) -> Result<crate::lib::db::models::Project, String> {
    let db = db.lock().await;
    crud::ProjectRepository::get_by_id(db.pool(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_project(
    db: State<'_, Arc<Mutex<Database>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<crate::lib::db::models::Project, String> {
    let db = db.lock().await;
    crud::ProjectRepository::update(db.pool(), &id, name, description)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_project(
    db: State<'_, Arc<Mutex<Database>>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    crud::ProjectRepository::delete(db.pool(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_episodes(
    db: State<'_, Arc<Mutex<Database>>>,
    project_id: String,
) -> Result<Vec<crate::lib::db::models::Episode>, String> {
    let db = db.lock().await;
    crud::EpisodeRepository::list_by_project(db.pool(), &project_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_episode(
    db: State<'_, Arc<Mutex<Database>>>,
    project_id: String,
    name: String,
    content: Option<String>,
) -> Result<crate::lib::db::models::Episode, String> {
    let db = db.lock().await;
    crud::EpisodeRepository::create(db.pool(), project_id, name, content)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_episode(
    db: State<'_, Arc<Mutex<Database>>>,
    id: String,
) -> Result<crate::lib::db::models::Episode, String> {
    let db = db.lock().await;
    crud::EpisodeRepository::get_by_id(db.pool(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_episode(
    db: State<'_, Arc<Mutex<Database>>>,
    id: String,
    name: Option<String>,
    content: Option<String>,
) -> Result<crate::lib::db::models::Episode, String> {
    let db = db.lock().await;
    crud::EpisodeRepository::update(db.pool(), &id, name, content)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_episode(
    db: State<'_, Arc<Mutex<Database>>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    crud::EpisodeRepository::delete(db.pool(), &id)
        .await
        .map_err(|e| e.to_string())
}
