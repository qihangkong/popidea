use crate::task::queue::QueuedTask;
use crate::errors::Result;

pub async fn handle_panel_video(task: &QueuedTask) -> Result<String> {
    Ok("Panel video generated".to_string())
}
