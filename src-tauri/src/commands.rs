use tauri::State;

#[tauri::command]
pub async fn get_projects() -> Result<Vec<String>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn create_project(name: String) -> Result<String, String> {
    Ok(name)
}
