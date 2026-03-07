use std::sync::Arc;
use tokio::sync::Mutex;
use crate::task::queue::{TaskQueue, QueuedTask, TaskStatus};
use crate::task::handlers;
use crate::errors::Result;

pub struct TaskWorker {
    queue: Arc<TaskQueue>,
    running: Arc<Mutex<bool>>,
}

impl TaskWorker {
    pub fn new(queue: Arc<TaskQueue>) -> Self {
        TaskWorker {
            queue,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        {
            let mut running = self.running.lock().await;
            if *running {
                return Ok(());
            }
            *running = true;
        }

        loop {
            {
                let running = self.running.lock().await;
                if !*running {
                    break;
                }
            }

            if let Some(task) = self.queue.dequeue().await {
                self.process_task(task).await?;
            } else {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        Ok(())
    }

    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }

    pub fn is_running(&self) -> bool {
        let running = self.running.try_lock();
        if let Ok(r) = running {
            *r
        } else {
            false
        }
    }

    async fn process_task(&self, task: QueuedTask) -> Result<()> {
        println!("Processing task: {} ({})", task.id, task.task_type);

        let result = match task.task_type.as_str() {
            "novel_analysis" => handlers::text::handle_novel_analysis(&task).await,
            "global_analysis" => handlers::text::handle_global_analysis(&task).await,
            "story_to_script" => handlers::text::handle_story_to_script(&task).await,
            "script_to_storyboard" => handlers::text::handle_script_to_storyboard(&task).await,
            "generate_character_image" => handlers::image::handle_character_image(&task).await,
            "generate_location_image" => handlers::image::handle_location_image(&task).await,
            "generate_panel_image" => handlers::image::handle_panel_image(&task).await,
            "generate_panel_video" => handlers::video::handle_panel_video(&task).await,
            "generate_voice" => handlers::voice::handle_voice_generation(&task).await,
            _ => Err(crate::errors::AppError::Custom(format!("Unknown task type: {}", task.task_type))),
        };

        match result {
            Ok(output) => {
                self.queue.complete_task(&task.id, Some(output)).await?;
                println!("Task completed: {}", task.id);
            }
            Err(e) => {
                self.queue.fail_task(&task.id, e.to_string()).await?;
                println!("Task failed: {} - {}", task.id, e);
            }
        }

        Ok(())
    }
}
