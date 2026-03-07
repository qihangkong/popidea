#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;

use std::sync::Arc;
use tokio::sync::RwLock;
use lib::commands;
use lib::db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle();
            
            tauri::async_runtime::spawn(async move {
                let pool = db::create_pool(None).await.expect("Failed to create database pool");
                db::init_database(&pool).await.expect("Failed to initialize database");
                
                let db_pool = Arc::new(RwLock::new(Some(pool)));
                
                let _ = handle.manage(db_pool);
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_projects,
            commands::create_project,
            commands::get_project,
            commands::update_project,
            commands::delete_project,
            commands::get_episodes,
            commands::create_episode,
            commands::import_episode,
            commands::get_episode,
            commands::update_episode,
            commands::delete_episode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
