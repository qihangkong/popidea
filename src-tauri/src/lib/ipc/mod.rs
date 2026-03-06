use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use crate::lib::{
    db::Database,
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
        app_handle.on_menu_event(|_app_handle, _event| {
            tracing::info!("Menu event received");
        });
    }
}
