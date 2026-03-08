use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::db::crud;
use crate::db::asset_crud;
use crate::db::models::*;
use crate::errors::{Result, AppError};

// ==================== Character Appearances ====================

#[tauri::command]
pub async fn get_character_appearances(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    character_id: String,
) -> Result<Vec<CharacterAppearance>, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::get_character_appearances(pool, &character_id).await {
            Ok(appearances) => Ok(appearances),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_character_appearance(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    character_id: String,
    description: Option<String>,
    image_url: Option<String>,
) -> Result<CharacterAppearance, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::create_character_appearance(pool, character_id, description, image_url).await {
            Ok(appearance) => Ok(appearance),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_character_appearance(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    description: Option<String>,
    image_url: Option<String>,
    is_selected: Option<bool>,
) -> Result<Option<CharacterAppearance>, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::update_character_appearance(pool, &id, description, image_url, is_selected).await {
            Ok(appearance) => Ok(appearance),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_character_appearance(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::delete_character_appearance(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn select_character_appearance(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    character_id: String,
    appearance_id: String,
) -> Result<(), String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::select_character_appearance(pool, &character_id, &appearance_id).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

// ==================== Asset Folders ====================

#[tauri::command]
pub async fn get_asset_folders(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    parent_id: Option<String>,
) -> Result<Vec<AssetFolder>, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::get_asset_folders(pool, &project_id, parent_id.as_deref()).await {
            Ok(folders) => Ok(folders),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_asset_folder(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    name: String,
    parent_id: Option<String>,
) -> Result<AssetFolder, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::create_asset_folder(pool, project_id, name, parent_id).await {
            Ok(folder) => Ok(folder),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_asset_folder(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    parent_id: Option<String>,
) -> Result<Option<AssetFolder>, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::update_asset_folder(pool, &id, name, parent_id).await {
            Ok(folder) => Ok(folder),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_asset_folder(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::delete_asset_folder(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

// ==================== Assets ====================

#[tauri::command]
pub async fn get_assets(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    folder_id: Option<String>,
    asset_type: Option<String>,
) -> Result<Vec<Asset>, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::get_assets(pool, &project_id, folder_id.as_deref(), asset_type.as_deref()).await {
            Ok(assets) => Ok(assets),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn create_asset(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    project_id: String,
    asset_type: String,
    folder_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    metadata: Option<String>,
    labels: Option<String>,
) -> Result<Asset, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::create_asset(pool, project_id, asset_type, folder_id, name, description, image_url, metadata, labels).await {
            Ok(asset) => Ok(asset),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn update_asset(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    metadata: Option<String>,
    labels: Option<String>,
) -> Result<Option<Asset>, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::update_asset(pool, &id, name, description, image_url, metadata, labels).await {
            Ok(asset) => Ok(asset),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}

#[tauri::command]
pub async fn delete_asset(
    db_pool: State<Arc<RwLock<Option<sqlx::SqlitePool>>>>,
    id: String,
) -> Result<bool, String> {
    let pool = db_pool.read().await;

    if let Some(pool) = pool.as_ref() {
        match asset_crud::delete_asset(pool, &id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Database pool not initialized".to_string())
    }
}
