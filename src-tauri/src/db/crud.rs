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
