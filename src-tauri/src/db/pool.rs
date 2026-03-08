use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::PathBuf;
use crate::errors::Result;

pub async fn create_pool(database_path: Option<PathBuf>) -> Result<SqlitePool> {
    let path = database_path.unwrap_or_else(|| {
        std::env::var("POPIdea_DB_PATH")
            .ok()
            .and_then(|p| PathBuf::from(p).canonicalize().ok())
            .unwrap_or_else(|| {
                let mut data_dir = std::env::current_dir()
                    .expect("Failed to get current directory");
                data_dir.push("popidea.db");
                data_dir
            })
    });

    let connection_string = format!("sqlite:{}", path.to_string_lossy());

    let pool = SqlitePool::connect(&connection_string).await?;

    Ok(pool)
}

pub async fn init_database(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            settings TEXT
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS episodes (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            content TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS storyboards (
            id TEXT PRIMARY KEY,
            episode_id TEXT NOT NULL,
            name TEXT NOT NULL,
            panel_count INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (episode_id) REFERENCES episodes(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS panels (
            id TEXT PRIMARY KEY,
            storyboard_id TEXT NOT NULL,
            panel_index INTEGER NOT NULL,
            panel_number INTEGER NOT NULL,
            shot_type TEXT,
            camera_move TEXT,
            description TEXT,
            location TEXT,
            characters TEXT,
            srt_start REAL,
            srt_end REAL,
            duration REAL,
            video_prompt TEXT,
            image_url TEXT,
            video_url TEXT,
            FOREIGN KEY (storyboard_id) REFERENCES storyboards(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS global_characters (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            appearances TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS global_locations (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            status TEXT NOT NULL,
            project TEXT NOT NULL,
            episode_id TEXT,
            target_type TEXT,
            target_id TEXT,
            payload TEXT,
            result TEXT,
            progress INTEGER DEFAULT 0,
            error_message TEXT,
            created_at INTEGER NOT NULL,
            started_at INTEGER,
            finished_at INTEGER
        )
        "#
    )
    .execute(pool)
    .await?;

    // 角色外貌表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS character_appearances (
            id TEXT PRIMARY KEY,
            character_id TEXT NOT NULL,
            description TEXT,
            image_url TEXT,
            is_selected INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (character_id) REFERENCES global_characters(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // 资产文件夹表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS asset_folders (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            parent_id TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // 资产表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS assets (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            folder_id TEXT,
            asset_type TEXT NOT NULL,
            name TEXT,
            description TEXT,
            image_url TEXT,
            metadata TEXT,
            labels TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (folder_id) REFERENCES asset_folders(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
