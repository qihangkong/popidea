pub mod models;
pub mod pool;
pub mod crud;

use crate::errors::{AppError, Result};
use sqlx::SqlitePool;
use std::path::Path;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let connection_string = format!("sqlite:{}?mode=rwc", db_path.as_ref().display());
        
        let pool = SqlitePool::connect(&connection_string).await?;
        
        let db = Database { pool };
        db.run_migrations().await?;
        
        Ok(db)
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    async fn run_migrations(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                settings TEXT
            );

            CREATE TABLE IF NOT EXISTS episodes (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                content TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS storyboards (
                id TEXT PRIMARY KEY,
                episode_id TEXT NOT NULL,
                name TEXT NOT NULL,
                panel_count INTEGER DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (episode_id) REFERENCES episodes(id)
            );

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
            );

            CREATE TABLE IF NOT EXISTS global_characters (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                appearances TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS global_locations (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

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
            );
            "#
        )
        .execute(self.pool())
        .await?;

        Ok(())
    }
}
