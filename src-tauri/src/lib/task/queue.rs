use crate::errors::Result;
use crate::lib::db::models::Task;
use std::collections::VecDeque;
use tokio::sync::Mutex;

pub struct TaskQueue {
    queue: Mutex<VecDeque<Task>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub async fn push(&self, task: Task) -> Result<()> {
        let mut queue = self.queue.lock().await;
        queue.push_back(task);
        Ok(())
    }

    pub async fn pop(&self) -> Option<Task> {
        let mut queue = self.queue.lock().await;
        queue.pop_front()
    }

    pub async fn is_empty(&self) -> bool {
        let queue = self.queue.lock().await;
        queue.is_empty()
    }

    pub async fn len(&self) -> usize {
        let queue = self.queue.lock().await;
        queue.len()
    }
}
