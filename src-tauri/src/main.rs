#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod
lib;

use std::sync::Arc;
use tokio::sync::RwLock;
use lib::commands;
use lib::db;
use lib::task;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_queue = Arc::new(task::TaskQueue::new());
    let task_queue_clone = task_queue.clone();

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

            let _ = app.manage(task_queue_clone);

            let worker = task::TaskWorker::new(task_queue);
            let worker_handle = app.handle();
            
            tauri::async_runtime::spawn(async move {
                if let Err(e) = worker.start().await {
                    eprintln!("Worker error: {}", e);
                }
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
            commands::create_task,
            commands::get_tasks,
            commands::get_task,
            commands::get_storyboards,
            commands::create_storyboard,
            commands::get_panels,
            commands::create_panel,
            commands::update_panel,
            commands::delete_panel,
            commands::get_global_characters,
            commands::create_global_character,
            commands::update_global_character,
            commands::delete_global_character,
            commands::get_global_locations,
            commands::create_global_location,
            commands::update_global_location,
            commands::delete_global_location,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
