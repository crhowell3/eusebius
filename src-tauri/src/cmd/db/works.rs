use sqlx::QueryBuilder;
use tauri::State;

use models::Work;

use super::DbState;

#[tauri::command]
pub async fn get_works(db: State<'_, DbState>) -> Result<Vec<Work>, String> {
    sqlx::query_as::<_, Work>("SELECT work_code, description, category_tag FROM works")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_work(db: State<'_, DbState>, work: Work) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO works (work_code, description, category_tag)
        VALUES (?, ?, ?)",
    )
    .bind(&work.work_code)
    .bind(&work.description)
    .bind(&work.category_tag)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_works(db: State<'_, DbState>, work_codes: Vec<String>) -> Result<(), String> {
    if work_codes.is_empty() {
        return Ok(());
    }

    let mut builder = QueryBuilder::new("DELETE FROM works WHERE work_code IN (");

    let mut separated = builder.separated(", ");
    for code in &work_codes {
        separated.push_bind(code);
    }
    separated.push_unseparated(")");

    builder
        .build()
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
