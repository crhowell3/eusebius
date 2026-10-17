use sqlx::QueryBuilder;
use tauri::Manager;

use models::Category;

use super::DbState;

/// # Errors
///
/// Returns an error if the categories cannot be retrieved.
#[tauri::command]
pub async fn get_categories(app: tauri::AppHandle) -> Result<Vec<Category>, String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query_as::<_, Category>(
        "SELECT * FROM categories ORDER BY
            CASE WHEN tag = 'MISC' THEN 1 ELSE 0 END,  -- MISC always last
            length(tag), tag",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())
}

/// # Errors
///
/// Returns an error if the category cannot be added.
#[tauri::command]
pub async fn add_category(
    app: tauri::AppHandle,
    tag: String,
    name: String,
) -> Result<Category, String> {
    if tag.is_empty() || !tag.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Tag must contain only letters".to_string());
    }

    if name.trim().is_empty() {
        return Err("Category name cannot be empty".to_string());
    }

    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let tag = tag.to_uppercase();
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE tag = ?)")
        .bind(&tag)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if exists {
        return Err(format!("Tag \"{tag}\" is already in use"));
    }

    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO categories (tag, name) VALUES (?, ?) RETURNING id",
    )
    .bind(&tag)
    .bind(name.trim())
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Category {
        id,
        tag,
        name: name.trim().to_string(),
    })
}

/// # Errors
///
/// Returns an error if the category cannot be updated.
#[tauri::command]
pub async fn update_category(
    app: tauri::AppHandle,
    id: i64,
    tag: String,
    name: String,
) -> Result<(), String> {
    if tag.is_empty() || !tag.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Tag must contain only letters".to_string());
    }

    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let tag = tag.to_uppercase();

    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE tag = ? AND id != ?)")
            .bind(&tag)
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|e| e.to_string())?;

    if exists {
        return Err(format!("Tag \"{tag}\" is already in use."));
    }

    sqlx::query("UPDATE categories SET tag = ?, name = ? WHERE id = ?")
        .bind(&tag)
        .bind(&name)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the categories cannot be deleted.
#[tauri::command]
pub async fn delete_categories(app: tauri::AppHandle, ids: Vec<i64>) -> Result<(), String> {
    if ids.is_empty() {
        return Ok(());
    }

    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let misc_id: i64 = sqlx::query_scalar("SELECT id FROM categories WHERE tag = 'MISC'")
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if ids.contains(&misc_id) {
        return Err("The Miscellaneous category cannot be deleted.".to_string());
    }

    for id in &ids {
        sqlx::query("UPDATE works SET category_id = ? WHERE category_id = ?")
            .bind(misc_id)
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    let mut builder = QueryBuilder::new("DELETE FROM categories WHERE id IN (");
    let mut separated = builder.separated(", ");
    for id in &ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    builder
        .build()
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
