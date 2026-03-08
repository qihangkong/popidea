use std::sync::Arc;
use crate::task::queue::TaskQueue;
use crate::errors::Result;

pub struct TaskWorker {
    queue: Arc<TaskQueue>,
}

impl TaskWorker {
    pub fn new(queue: Arc<TaskQueue>) -> Self {
        TaskWorker { queue }
    }

    pub async fn start(&self) -> Result<()> {
        loop {
            if let Some(_task) = self.queue.dequeue().await {
                // Task processing logic will be implemented here
                eprintln!("Task received");
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
