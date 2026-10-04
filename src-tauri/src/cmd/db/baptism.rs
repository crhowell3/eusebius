use sqlx::QueryBuilder;
use tauri::Manager;

use models::Baptism;

use super::DbState;

/// # Errors
///
/// Returns an error if the baptisms cannot be retrieved.
#[tauri::command]
pub async fn get_baptisms(app: tauri::AppHandle) -> Result<Vec<Baptism>, String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query_as::<_, Baptism>("SELECT * FROM baptisms")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
}

/// # Errors
///
/// Returns an error if the baptisms cannot be retrieved.
#[tauri::command]
pub async fn add_baptism(app: tauri::AppHandle, baptism: Baptism) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query(
        "INSERT INTO baptisms (family_id, last_name, first_name, date_baptized, witness, location)
        VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&baptism.family_id)
    .bind(&baptism.last_name)
    .bind(&baptism.first_name)
    .bind(&baptism.date_baptized)
    .bind(&baptism.witness)
    .bind(&baptism.location)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the baptisms cannot be updated.
#[tauri::command]
pub async fn update_baptism(app: tauri::AppHandle, baptism: Baptism) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query("UPDATE baptisms SET last_name = ?, first_name = ?, date_baptized = ?, witness = ?, location = ? WHERE family_id = ?")
        .bind(&baptism.last_name)
        .bind(&baptism.first_name)
        .bind(&baptism.date_baptized)
        .bind(&baptism.witness)
        .bind(&baptism.location)
        .bind(&baptism.family_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the baptisms cannot be deleted.
#[tauri::command]
pub async fn delete_baptisms(app: tauri::AppHandle, family_ids: Vec<String>) -> Result<(), String> {
    if family_ids.is_empty() {
        return Ok(());
    }

    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let mut builder = QueryBuilder::new("DELETE FROM baptisms WHERE family_id IN (");

    let mut separated = builder.separated(", ");
    for ids in &family_ids {
        separated.push_bind(ids);
    }
    separated.push_unseparated(")");

    builder
        .build()
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
