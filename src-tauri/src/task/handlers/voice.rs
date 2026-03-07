use crate::task::queue::QueuedTask;
use crate::errors::Result;

pub async fn handle_voice_generation(task: &QueuedTask) -> Result<String> {
    Ok("Voice generated".to_string())
}
