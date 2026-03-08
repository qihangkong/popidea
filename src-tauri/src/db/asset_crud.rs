use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::db::models::*;
use crate::errors::Result;

// ==================== Character Appearances ====================

pub async fn create_character_appearance(
    pool: &SqlitePool,
    character_id: String,
    description: Option<String>,
    image_url: Option<String>,
) -> Result<CharacterAppearance> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let appearance = sqlx::query_as::<_, CharacterAppearance>(
        r#"
        INSERT INTO character_appearances (id, character_id, description, image_url, is_selected, created_at)
        VALUES (?1, ?2, ?3, ?4, 0, ?5)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&character_id)
    .bind(&description)
    .bind(&image_url)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(appearance)
}

pub async fn get_character_appearances(
    pool: &SqlitePool,
    character_id: &str,
) -> Result<Vec<CharacterAppearance>> {
    let appearances = sqlx::query_as::<_, CharacterAppearance>(
        "SELECT * FROM character_appearances WHERE character_id = ?1 ORDER BY created_at DESC"
    )
    .bind(character_id)
    .fetch_all(pool)
    .await?;

    Ok(appearances)
}

pub async fn get_character_appearance(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<CharacterAppearance>> {
    let appearance = sqlx::query_as::<_, CharacterAppearance>(
        "SELECT * FROM character_appearances WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(appearance)
}

pub async fn update_character_appearance(
    pool: &SqlitePool,
    id: &str,
    description: Option<String>,
    image_url: Option<String>,
    is_selected: Option<bool>,
) -> Result<Option<CharacterAppearance>> {
    let appearance = sqlx::query_as::<_, CharacterAppearance>(
        r#"
        UPDATE character_appearances
        SET description = COALESCE(?1, description),
            image_url = COALESCE(?2, image_url),
            is_selected = COALESCE(?3, is_selected)
        WHERE id = ?4
        RETURNING *
        "#
    )
    .bind(&description)
    .bind(&image_url)
    .bind(is_selected)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(appearance)
}

pub async fn delete_character_appearance(
    pool: &SqlitePool,
    id: &str,
) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM character_appearances WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn select_character_appearance(
    pool: &SqlitePool,
    character_id: &str,
    appearance_id: &str,
) -> Result<()> {
    // Deselect all appearances for this character
    sqlx::query(
        "UPDATE character_appearances SET is_selected = 0 WHERE character_id = ?1"
    )
    .bind(character_id)
    .execute(pool)
    .await?;

    // Select the specified appearance
    sqlx::query(
        "UPDATE character_appearances SET is_selected = 1 WHERE id = ?1"
    )
    .bind(appearance_id)
    .execute(pool)
    .await?;

    Ok(())
}

// ==================== Asset Folders ====================

pub async fn create_asset_folder(
    pool: &SqlitePool,
    project_id: String,
    name: String,
    parent_id: Option<String>,
) -> Result<AssetFolder> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let folder = sqlx::query_as::<_, AssetFolder>(
        r#"
        INSERT INTO asset_folders (id, project_id, name, parent_id, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&project_id)
    .bind(&name)
    .bind(&parent_id)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(folder)
}

pub async fn get_asset_folders(
    pool: &SqlitePool,
    project_id: &str,
    parent_id: Option<&str>,
) -> Result<Vec<AssetFolder>> {
    let folders = if let Some(parent) = parent_id {
        sqlx::query_as::<_, AssetFolder>(
            "SELECT * FROM asset_folders WHERE project_id = ?1 AND parent_id = ?2 ORDER BY created_at ASC"
        )
        .bind(project_id)
        .bind(parent)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, AssetFolder>(
            "SELECT * FROM asset_folders WHERE project_id = ?1 AND parent_id IS NULL ORDER BY created_at ASC"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?
    };

    Ok(folders)
}

pub async fn get_asset_folder(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<AssetFolder>> {
    let folder = sqlx::query_as::<_, AssetFolder>(
        "SELECT * FROM asset_folders WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(folder)
}

pub async fn update_asset_folder(
    pool: &SqlitePool,
    id: &str,
    name: Option<String>,
    parent_id: Option<String>,
) -> Result<Option<AssetFolder>> {
    let now = Utc::now().timestamp();

    let folder = sqlx::query_as::<_, AssetFolder>(
        r#"
        UPDATE asset_folders
        SET name = COALESCE(?1, name),
            parent_id = COALESCE(?2, parent_id),
            updated_at = ?3
        WHERE id = ?4
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(&parent_id)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(folder)
}

pub async fn delete_asset_folder(
    pool: &SqlitePool,
    id: &str,
) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM asset_folders WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

// ==================== Assets ====================

pub async fn create_asset(
    pool: &SqlitePool,
    project_id: String,
    asset_type: String,
    folder_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    metadata: Option<String>,
    labels: Option<String>,
) -> Result<Asset> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let asset = sqlx::query_as::<_, Asset>(
        r#"
        INSERT INTO assets (id, project_id, folder_id, asset_type, name, description, image_url, metadata, labels, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&project_id)
    .bind(&folder_id)
    .bind(&asset_type)
    .bind(&name)
    .bind(&description)
    .bind(&image_url)
    .bind(&metadata)
    .bind(&labels)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(asset)
}

pub async fn get_assets(
    pool: &SqlitePool,
    project_id: &str,
    folder_id: Option<&str>,
    asset_type: Option<&str>,
) -> Result<Vec<Asset>> {
    let assets = if let Some(folder) = folder_id {
        if let Some(atype) = asset_type {
            sqlx::query_as::<_, Asset>(
                "SELECT * FROM assets WHERE project_id = ?1 AND folder_id = ?2 AND asset_type = ?3 ORDER BY created_at DESC"
            )
            .bind(project_id)
            .bind(folder)
            .bind(atype)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, Asset>(
                "SELECT * FROM assets WHERE project_id = ?1 AND folder_id = ?2 ORDER BY created_at DESC"
            )
            .bind(project_id)
            .bind(folder)
            .fetch_all(pool)
            .await?
        }
    } else {
        if let Some(atype) = asset_type {
            sqlx::query_as::<_, Asset>(
                "SELECT * FROM assets WHERE project_id = ?1 AND asset_type = ?2 ORDER BY created_at DESC"
            )
            .bind(project_id)
            .bind(atype)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, Asset>(
                "SELECT * FROM assets WHERE project_id = ?1 ORDER BY created_at DESC"
            )
            .bind(project_id)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(assets)
}

pub async fn get_asset(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<Asset>> {
    let asset = sqlx::query_as::<_, Asset>(
        "SELECT * FROM assets WHERE id = ?1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(asset)
}

pub async fn update_asset(
    pool: &SqlitePool,
    id: &str,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    metadata: Option<String>,
    labels: Option<String>,
) -> Result<Option<Asset>> {
    let now = Utc::now().timestamp();

    let asset = sqlx::query_as::<_, Asset>(
        r#"
        UPDATE assets
        SET name = COALESCE(?1, name),
            description = COALESCE(?2, description),
            image_url = COALESCE(?3, image_url),
            metadata = COALESCE(?4, metadata),
            labels = COALESCE(?5, labels),
            updated_at = ?6
        WHERE id = ?7
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(&description)
    .bind(&image_url)
    .bind(&metadata)
    .bind(&labels)
    .bind(now)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(asset)
}

pub async fn delete_asset(
    pool: &SqlitePool,
    id: &str,
) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM assets WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
