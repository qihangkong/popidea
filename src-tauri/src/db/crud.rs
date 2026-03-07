use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::db::models::*;
use crate::errors::Result;

pub async fn create_project(pool: &SqlitePool, name: String, description: Option<String>) -> Result<Project> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (id, name, description, created_at, updated_at, settings)
        VALUES (?1, ?2, ?3, ?4, ?5, NULL)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&name)
    .bind(&description)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(project)
}

pub async fn get_projects(pool: &SqlitePool) -> Result<Vec<Project>> {
    let projects = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn get_project(pool: &SqlitePool, id: &str) -> Result<Option<Project>> {
    let project = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(project)
}

pub async fn update_project(pool: &SqlitePool, id: &str, name: Option<String>, description: Option<String>) -> Result<Option<Project>> {
    let now = Utc::now().timestamp();

    let project = sqlx::query_as::<_, Project>(
        r#"
        UPDATE projects
        SET name = COALESCE(?1, name),
            description = COALESCE(?2, description),
            updated_at = ?3
        WHERE id = ?4
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(&description)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(project)
}

pub async fn delete_project(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM projects WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn create_episode(pool: &SqlitePool, project_id: String, name: String, content: Option<String>) -> Result<Episode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let episode = sqlx::query_as::<_, Episode>(
        r#"
        INSERT INTO episodes (id, project_id, name, content, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&project_id)
    .bind(&name)
    .bind(&content)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(episode)
}

pub async fn get_episodes(pool: &SqlitePool, project_id: &str) -> Result<Vec<Episode>> {
    let episodes = sqlx::query_as::<_, Episode>(
        "SELECT * FROM episodes WHERE project_id = ?1 ORDER BY created_at DESC"
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(episodes)
}

pub async fn get_episode(pool: &SqlitePool, id: &str) -> Result<Option<Episode>> {
    let episode = sqlx::query_as::<_, Episode>(
        "SELECT * FROM episodes WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(episode)
}

pub async fn update_episode(pool: &SqlitePool, id: &str, name: Option<String>, content: Option<String>) -> Result<Option<Episode>> {
    let now = Utc::now().timestamp();

    let episode = sqlx::query_as::<_, Episode>(
        r#"
        UPDATE episodes
        SET name = COALESCE(?1, name),
            content = COALESCE(?2, content),
            updated_at = ?3
        WHERE id = ?4
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(&content)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(episode)
}

pub async fn delete_episode(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM episodes WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn create_storyboard(pool: &SqlitePool, episode_id: String, name: String) -> Result<Storyboard> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let storyboard = sqlx::query_as::<_, Storyboard>(
        r#"
        INSERT INTO storyboards (id, episode_id, name, panel_count, created_at, updated_at)
        VALUES (?1, ?2, ?3, 0, ?4, ?5)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&episode_id)
    .bind(&name)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(storyboard)
}

pub async fn get_storyboards(pool: &SqlitePool, episode_id: &str) -> Result<Vec<Storyboard>> {
    let storyboards = sqlx::query_as::<_, Storyboard>(
        "SELECT * FROM storyboards WHERE episode_id = ?1 ORDER BY created_at DESC"
    )
    .bind(episode_id)
    .fetch_all(pool)
    .await?;

    Ok(storyboards)
}

pub async fn get_storyboard(pool: &SqlitePool, id: &str) -> Result<Option<Storyboard>> {
    let storyboard = sqlx::query_as::<_, Storyboard>(
        "SELECT * FROM storyboards WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(storyboard)
}

pub async fn update_storyboard(pool: &SqlitePool, id: &str, name: Option<String>) -> Result<Option<Storyboard>> {
    let now = Utc::now().timestamp();

    let storyboard = sqlx::query_as::<_, Storyboard>(
        r#"
        UPDATE storyboards
        SET name = COALESCE(?1, name),
            updated_at = ?2
        WHERE id = ?3
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(storyboard)
}

pub async fn delete_storyboard(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM storyboards WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn create_panel(pool: &SqlitePool, storyboard_id: String, panel_number: i32) -> Result<Panel> {
    let id = Uuid::new_v4().to_string();
    
    let panel_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM panels WHERE storyboard_id = ?1"
    )
    .bind(&storyboard_id)
    .fetch_one(pool)
    .await?;

    let panel = sqlx::query_as::<_, Panel>(
        r#"
        INSERT INTO panels (id, storyboard_id, panel_index, panel_number, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&storyboard_id)
    .bind(panel_count as i32)
    .bind(panel_number)
    .bind(Utc::now().timestamp())
    .bind(Utc::now().timestamp())
    .fetch_one(pool)
    .await?;

    Ok(panel)
}

pub async fn get_panels(pool: &SqlitePool, storyboard_id: &str) -> Result<Vec<Panel>> {
    let panels = sqlx::query_as::<_, Panel>(
        "SELECT * FROM panels WHERE storyboard_id = ?1 ORDER BY panel_index ASC"
    )
    .bind(storyboard_id)
    .fetch_all(pool)
    .await?;

    Ok(panels)
}

pub async fn get_panel(pool: &SqlitePool, id: &str) -> Result<Option<Panel>> {
    let panel = sqlx::query_as::<_, Panel>(
        "SELECT * FROM panels WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(panel)
}

pub async fn update_panel(pool: &SqlitePool, id: &str, updates: PanelUpdate) -> Result<Option<Panel>> {
    let now = Utc::now().timestamp();

    let panel = sqlx::query_as::<_, Panel>(
        r#"
        UPDATE panels
        SET shot_type = COALESCE(?1, shot_type),
            camera_move = COALESCE(?2, camera_move),
            description = COALESCE(?3, description),
            location = COALESCE(?4, location),
            characters = COALESCE(?5, characters),
            srt_start = COALESCE(?6, srt_start),
            srt_end = COALESCE(?7, srt_end),
            duration = COALESCE(?8, duration),
            video_prompt = COALESCE(?9, video_prompt),
            image_url = COALESCE(?10, image_url),
            video_url = COALESCE(?11, video_url),
            updated_at = ?12
        WHERE id = ?13
        RETURNING *
        "#
    )
    .bind(&updates.shot_type)
    .bind(&updates.camera_move)
    .bind(&updates.description)
    .bind(&updates.location)
    .bind(&updates.characters)
    .bind(&updates.srt_start)
    .bind(&updates.srt_end)
    .bind(&updates.duration)
    .bind(&updates.video_prompt)
    .bind(&updates.image_url)
    .bind(&updates.video_url)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(panel)
}

pub async fn delete_panel(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM panels WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub struct PanelUpdate {
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

pub async fn create_global_character(pool: &SqlitePool, project_id: String, name: String, description: Option<String>) -> Result<GlobalCharacter> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let character = sqlx::query_as::<_, GlobalCharacter>(
        r#"
        INSERT INTO global_characters (id, project_id, name, description, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&project_id)
    .bind(&name)
    .bind(&description)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(character)
}

pub async fn get_global_characters(pool: &SqlitePool, project_id: &str) -> Result<Vec<GlobalCharacter>> {
    let characters = sqlx::query_as::<_, GlobalCharacter>(
        "SELECT * FROM global_characters WHERE project_id = ?1 ORDER BY created_at DESC"
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(characters)
}

pub async fn get_global_character(pool: &SqlitePool, id: &str) -> Result<Option<GlobalCharacter>> {
    let character = sqlx::query_as::<_, GlobalCharacter>(
        "SELECT * FROM global_characters WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(character)
}

pub async fn update_global_character(pool: &SqlitePool, id: &str, name: Option<String>, description: Option<String>) -> Result<Option<GlobalCharacter>> {
    let now = Utc::now().timestamp();

    let character = sqlx::query_as::<_, GlobalCharacter>(
        r#"
        UPDATE global_characters
        SET name = COALESCE(?1, name),
            description = COALESCE(?2, description),
            updated_at = ?3
        WHERE id = ?4
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(&description)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(character)
}

pub async fn delete_global_character(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM global_characters WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn create_global_location(pool: &SqlitePool, project_id: String, name: String, description: Option<String>) -> Result<GlobalLocation> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let location = sqlx::query_as::<_, GlobalLocation>(
        r#"
        INSERT INTO global_locations (id, project_id, name, description, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&project_id)
    .bind(&name)
    .bind(&description)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(location)
}

pub async fn get_global_locations(pool: &SqlitePool, project_id: &str) -> Result<Vec<GlobalLocation>> {
    let locations = sqlx::query_as::<_, GlobalLocation>(
        "SELECT * FROM global_locations WHERE project_id = ?1 ORDER BY created_at DESC"
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(locations)
}

pub async fn get_global_location(pool: &SqlitePool, id: &str) -> Result<Option<GlobalLocation>> {
    let location = sqlx::query_as::<_, GlobalLocation>(
        "SELECT * FROM global_locations WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(location)
}

pub async fn update_global_location(pool: &SqlitePool, id: &str, name: Option<String>, description: Option<String>) -> Result<Option<GlobalLocation>> {
    let now = Utc::now().timestamp();

    let location = sqlx::query_as::<_, GlobalLocation>(
        r#"
        UPDATE global_locations
        SET name = COALESCE(?1, name),
            description = COALESCE(?2, description),
            updated_at = ?3
        WHERE id = ?4
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(&description)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(location)
}

pub async fn delete_global_location(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM global_locations WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
