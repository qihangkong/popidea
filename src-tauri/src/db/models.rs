use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub settings: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Episode {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub content: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Storyboard {
    pub id: String,
    pub episode_id: String,
    pub name: String,
    pub panel_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Panel {
    pub id: String,
    pub storyboard_id: String,
    pub panel_index: i32,
    pub panel_number: i32,
    pub shot_type: Option<String>,
    pub camera_move: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub characters: Option<String>,
    pub srt_start: Option<f64>,
    pub srt_end: Option<f64>,
    pub duration: Option<f64>,
    pub video_prompt: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GlobalCharacter {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub appearances: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GlobalLocation {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub status: String,
    pub project: String,
    pub episode_id: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub payload: Option<String>,
    pub result: Option<String>,
    pub progress: i32,
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CharacterAppearance {
    pub id: String,
    pub character_id: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub is_selected: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AssetFolder {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Asset {
    pub id: String,
    pub project_id: String,
    pub folder_id: Option<String>,
    pub asset_type: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub metadata: Option<String>,
    pub labels: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
