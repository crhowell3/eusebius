use sqlx::QueryBuilder;
use tauri::Manager;

use models::Death;

use super::DbState;

/// # Errors
///
/// Returns an error if the deaths cannot be retrieved.
#[tauri::command]
pub async fn get_deaths(app: tauri::AppHandle) -> Result<Vec<Death>, String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query_as::<_, Death>("SELECT * FROM deaths")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
}

/// # Errors
///
/// Returns an error if the death cannot be added.
#[tauri::command]
pub async fn add_death(app: tauri::AppHandle, death: Death) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query(
        "INSERT INTO deaths (first_name, last_name, date_of_death)
        VALUES (?, ?, ?)",
    )
    .bind(&death.first_name)
    .bind(&death.last_name)
    .bind(&death.date_of_death)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the death cannot be updated.
#[tauri::command]
pub async fn update_death(app: tauri::AppHandle, death: Death) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query("UPDATE deaths SET first_name = ?, last_name = ?, date_of_death = ? WHERE id = ?")
        .bind(&death.first_name)
        .bind(&death.last_name)
        .bind(&death.date_of_death)
        .bind(death.id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the deaths cannot be deleted.
#[tauri::command]
pub async fn delete_deaths(app: tauri::AppHandle, death_ids: Vec<i64>) -> Result<(), String> {
    if death_ids.is_empty() {
        return Ok(());
    }

    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let mut builder = QueryBuilder::new("DELETE FROM deaths WHERE id IN (");
    let mut separated = builder.separated(", ");
    for id in &death_ids {
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
