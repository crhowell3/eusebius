use sqlx::QueryBuilder;
use tauri::State;

use models::Work;

use super::DbState;

#[tauri::command]
pub async fn get_works(db: State<'_, DbState>) -> Result<Vec<Work>, String> {
    sqlx::query_as::<_, Work>(
        "SELECT w.id, w.category_tag, w.description, c.name AS category_name
        FROM works w
        LEFT JOIN categories c ON w.category_tag = c.tag
        ORDER BY w.category_tag
        ",
    )
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_work(db: State<'_, DbState>, work: Work) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO works (description, category_tag)
        VALUES (?, ?)",
    )
    .bind(&work.description)
    .bind(&work.category_tag)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_work(db: State<'_, DbState>, work: Work) -> Result<(), String> {
    sqlx::query("UPDATE works SET description = ?, category_tag = ? WHERE id = ?")
        .bind(&work.description)
        .bind(&work.category_tag)
        .bind(&work.id)
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_works(db: State<'_, DbState>, work_ids: Vec<i64>) -> Result<(), String> {
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
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
