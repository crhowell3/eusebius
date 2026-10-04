use sqlx::QueryBuilder;
use tauri::Manager;

use models::Work;

use super::DbState;

/// # Errors
///
/// Returns an error if the works cannot be retrieved.
#[tauri::command]
pub async fn get_works(app: tauri::AppHandle) -> Result<Vec<Work>, String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query_as::<_, Work>(
        "SELECT w.id, w.category_id, c.tag AS category_tag, w.description, c.name AS category_name
        FROM works w
        LEFT JOIN categories c ON w.category_id = c.id
        ORDER BY w.category_id
        ",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())
}

/// # Errors
///
/// Returns an error if the works cannot be retrieved.
#[tauri::command]
pub async fn add_work(app: tauri::AppHandle, work: Work) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query(
        "INSERT INTO works (description, category_id)
        VALUES (?, ?)",
    )
    .bind(&work.description)
    .bind(work.category_id)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the work cannot be updated.
#[tauri::command]
pub async fn update_work(app: tauri::AppHandle, work: Work) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query("UPDATE works SET description = ?, category_id = ? WHERE id = ?")
        .bind(&work.description)
        .bind(work.category_id)
        .bind(work.id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the works cannot be deleted.
#[tauri::command]
pub async fn delete_works(app: tauri::AppHandle, work_ids: Vec<i64>) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    if work_ids.is_empty() {
        return Ok(());
    }

    let mut builder = QueryBuilder::new("DELETE FROM works WHERE id IN (");

    let mut separated = builder.separated(", ");
    for id in &work_ids {
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
