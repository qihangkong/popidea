use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use serde_json::Value;
use crate::errors::Result;

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Queued => "queued",
            TaskStatus::Processing => "processing",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueuedTask {
    pub id: String,
    pub task_type: String,
    pub status: TaskStatus,
    pub project: String,
    pub episode_id: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub payload: Option<Value>,
    pub result: Option<String>,
    pub progress: i32,
    pub error_message: Option<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
}

pub struct TaskQueue {
    tasks: Arc<Mutex<Vec<QueuedTask>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn enqueue(&self, task: QueuedTask) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        tasks.push(task);
        Ok(())
    }

    pub async fn dequeue(&self) -> Option<QueuedTask> {
        let mut tasks = self.tasks.lock().await;

        for (i, task) in tasks.iter().enumerate() {
            if matches!(task.status, TaskStatus::Queued) {
                let task = task.clone();
                tasks[i].status = TaskStatus::Processing;
                tasks[i].started_at = Some(Utc::now().timestamp());
                return Some(task);
            }
        }

        None
    }

    pub async fn update_progress(&self, task_id: &str, progress: i32) -> Result<()> {
        let mut tasks = self.tasks.lock().await;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.progress = progress;
        }

        Ok(())
    }

    pub async fn complete_task(&self, task_id: &str, result: Option<String>) -> Result<()> {
        let mut tasks = self.tasks.lock().await;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = TaskStatus::Completed;
            task.result = result;
            task.finished_at = Some(Utc::now().timestamp());
        }

        Ok(())
    }

    pub async fn fail_task(&self, task_id: &str, error_message: String) -> Result<()> {
        let mut tasks = self.tasks.lock().await;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = TaskStatus::Failed;
            task.error_message = Some(error_message);
            task.finished_at = Some(Utc::now().timestamp());
        }

        Ok(())
    }

    pub async fn get_all_tasks(&self) -> Vec<QueuedTask> {
        let tasks = self.tasks.lock().await;
        tasks.clone()
    }

    pub async fn get_task(&self, task_id: &str) -> Option<QueuedTask> {
        let tasks = self.tasks.lock().await;
        tasks.iter().find(|t| t.id == task_id).cloned()
    }
}
