use crate::errors::Result;
use crate::lib::task::TaskQueue;
use tokio::sync::mpsc;

pub struct TaskWorker {
    queue: TaskQueue,
    receiver: mpsc::Receiver<String>,
}

impl TaskWorker {
    pub fn new(queue: TaskQueue) -> (Self, mpsc::Sender<String>) {
        let (sender, receiver) = mpsc::channel(100);
        
        let worker = Self {
            queue,
            receiver,
        };
        
        (worker, sender)
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(task_id) = self.receiver.recv().await {
            if let Some(task) = self.queue.pop().await {
                self.process_task(task).await?;
            }
        }
        Ok(())
    }

    async fn process_task(&self, task: crate::lib::db::models::Task) -> Result<()> {
        tracing::info!("Processing task: {}", task.id);
        Ok(())
    }
}
