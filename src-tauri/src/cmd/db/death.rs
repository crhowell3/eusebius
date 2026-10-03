use sqlx::QueryBuilder;
use tauri::State;

use models::Death;

use super::DbState;

/// # Errors
///
/// Returns an error if the deaths cannot be retrieved.
#[tauri::command]
pub async fn get_deaths(db: State<'_, DbState>) -> Result<Vec<Death>, String> {
    sqlx::query_as::<_, Death>("SELECT * FROM deaths")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

/// # Errors
///
/// Returns an error if the death cannot be added.
#[tauri::command]
pub async fn add_death(db: State<'_, DbState>, death: Death) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO deaths (first_name, last_name, date_of_death)
        VALUES (?, ?, ?)",
    )
    .bind(&death.first_name)
    .bind(&death.last_name)
    .bind(&death.date_of_death)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the death cannot be updated.
#[tauri::command]
pub async fn update_death(db: State<'_, DbState>, death: Death) -> Result<(), String> {
    sqlx::query("UPDATE deaths SET first_name = ?, last_name = ?, date_of_death = ? WHERE id = ?")
        .bind(&death.first_name)
        .bind(&death.last_name)
        .bind(&death.date_of_death)
        .bind(&death.id)
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the deaths cannot be deleted.
#[tauri::command]
pub async fn delete_deaths(db: State<'_, DbState>, death_ids: Vec<i64>) -> Result<(), String> {
    if death_ids.is_empty() {
        return Ok(());
    }
    let mut builder = QueryBuilder::new("DELETE FROM deaths WHERE id IN (");
    let mut separated = builder.separated(", ");
    for id in &death_ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");
    builder
        .build()
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
