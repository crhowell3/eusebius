use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Work {
    work_code: String,
    description: String,
}

pub struct DbState(pub SqlitePool);

#[tauri::command]
pub async fn get_works(db: State<'_, DbState>) -> Result<Vec<Work>, String> {
    sqlx::query_as::<_, Work>("SELECT work_code, description FROM works")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_work(db: State<'_, DbState>, work: Work) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO works (work_code, description)
        VALUES (?, ?)",
    )
    .bind(&work.work_code)
    .bind(&work.description)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
