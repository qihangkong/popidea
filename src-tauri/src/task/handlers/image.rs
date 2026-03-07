use crate::task::queue::QueuedTask;
use crate::errors::Result;

pub async fn handle_character_image(task: &QueuedTask) -> Result<String> {
    Ok("Character image generated".to_string())
}

pub async fn handle_location_image(task: &QueuedTask) -> Result<String> {
    Ok("Location image generated".to_string())
}

pub async fn handle_panel_image(task: &QueuedTask) -> Result<String> {
    Ok("Panel image generated".to_string())
}
