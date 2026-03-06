#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use tracing::{info, error};

mod lib;
mod errors;

use lib::{
    db::Database,
    storage::StorageService,
    task::TaskQueue,
    api::HttpServer,
    ipc::IpcHandler,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting NoIdea application...");

    let db = match Database::new("noidea.db").await {
        Ok(db) => {
            info!("Database initialized successfully");
            Arc::new(Mutex::new(db))
        }
        Err(e) => {
            error!("Failed to initialize database: {}", e);
            return;
        }
    };

    let storage = match StorageService::new("./storage").await {
        Ok(storage) => {
            info!("Storage service initialized successfully");
            Arc::new(Mutex::new(storage))
        }
        Err(e) => {
            error!("Failed to initialize storage service: {}", e);
            return;
        }
    };

    let task_queue = Arc::new(Mutex::new(TaskQueue::new()));

    let http_server = HttpServer::new(3000, db.clone(), storage.clone(), task_queue.clone());
    tokio::spawn(async move {
        if let Err(e) = http_server.start().await {
            error!("HTTP server error: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app app_handle| {
            let ipc_handler = IpcHandler::new(
                db.clone(),
                storage.clone(),
                task_queue.clone(),
            );
            ipc_handler.register(app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
