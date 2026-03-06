use crate::errors::{AppError, Result};
use crate::lib::db::models::{Project, Episode, Storyboard, Panel, GlobalCharacter, GlobalLocation, Task};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn create(pool: &SqlitePool, name: String, description: Option<String>) -> Result<Project> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO projects (id, name, description, created_at, updated_at, settings) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )
        .bind(&id)
        .bind(&name)
        .bind(&description)
        .bind(now)
        .bind(now)
        .bind(serde_json::to_string(&serde_json::json!({})).unwrap())
        .execute(pool)
        .await?;

        Self::get_by_id(pool, &id).await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Project> {
        let row = sqlx::query_as::<_, (String, String, Option<String>, i64, i64, Option<String>)>(
            "SELECT id, name, description, created_at, updated_at, settings FROM projects WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;

        Ok(Project {
            id: row.0,
            name: row.1,
            description: row.2,
            created_at: DateTime::from_timestamp(row.3, 0).unwrap(),
            updated_at: DateTime::from_timestamp(row.4, 0).unwrap(),
            settings: row.5.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Project>> {
        let rows = sqlx::query_as::<_, (String, String, Option<String>, i64, i64, Option<String>)>(
            "SELECT id, name, description, created_at, updated_at, settings FROM projects ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        let projects = rows.into_iter().map(|row| Project {
            id: row.0,
            name: row.1,
            description: row.2,
            created_at: DateTime::from_timestamp(row.3, 0).unwrap(),
            updated_at: DateTime::from_timestamp(row.4, 0).unwrap(),
            settings: row.5.and_then(|s| serde_json::from_str(&s).ok()),
        }).collect();

        Ok(projects)
    }

    pub async fn update(pool: &SqlitePool, id: &str, name: Option<String>, description: Option<String>) -> Result<Project> {
        let now = Utc::now();

        if let Some(name) = name {
            sqlx::query("UPDATE projects SET name = ?1, updated_at = ?2 WHERE id = ?3")
                .bind(&name)
                .bind(now)
                .bind(id)
                .execute(pool)
                .await?;
        }

        if let Some(description) = description {
            sqlx::query("UPDATE projects SET description = ?1, updated_at = ?2 WHERE id = ?3")
                .bind(&description)
                .bind(now)
                .bind(id)
                .execute(pool)
                .await?;
        }

        Self::get_by_id(pool, id).await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Project {} not found", id)));
        }

        Ok(())
    }
}

pub struct EpisodeRepository;

impl EpisodeRepository {
    pub async fn create(pool: &SqlitePool, project_id: String, name: String, content: Option<String>) -> Result<Episode> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO episodes (id, project_id, name, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )
        .bind(&id)
        .bind(&project_id)
        .bind(&name)
        .bind(&content)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        Self::get_by_id(pool, &id).await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Episode> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, i64, i64)>(
            "SELECT id, project_id, name, content, created_at, updated_at FROM episodes WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Episode {} not found", id)))?;

        Ok(Episode {
            id: row.0,
            project_id: row.1,
            name: row.2,
            content: row.3,
            created_at: DateTime::from_timestamp(row.4, 0).unwrap(),
            updated_at: DateTime::from_timestamp(row.5, 0).unwrap(),
        })
    }

    pub async fn list_by_project(pool: &SqlitePool, project_id: &str) -> Result<Vec<Episode>> {
        let rows = sqlx::query_as::<_, (String, String, String, Option<String>, i64, i64)>(
            "SELECT id, project_id, name, content, created_at, updated_at FROM episodes WHERE project_id = ?1 ORDER BY created_at DESC"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let episodes = rows.into_iter().map(|row| Episode {
            id: row.0,
            project_id: row.1,
            name: row.2,
            content: row.3,
            created_at: DateTime::from_timestamp(row.4, 0).unwrap(),
            updated_at: DateTime::from_timestamp(row.5, 0).unwrap(),
        }).collect();

        Ok(episodes)
    }

    pub async fn update(pool: &SqlitePool, id: &str, name: Option<String>, content: Option<String>) -> Result<Episode> {
        let now = Utc::now();

        if let Some(name) = name {
            sqlx::query("UPDATE episodes SET name = ?1, updated_at = ?2 WHERE id = ?3")
                .bind(&name)
                .bind(now)
                .bind(id)
                .execute(pool)
                .await?;
        }

        if let Some(content) = content {
            sqlx::query("UPDATE episodes SET content = ?1, updated_at = ?2 WHERE id = ?3")
                .bind(&content)
                .bind(now)
                .bind(id)
                .execute(pool)
                .await?;
        }

        Self::get_by_id(pool, id).await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM episodes WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Episode {} not found", id)));
        }

        Ok(())
    }
}
