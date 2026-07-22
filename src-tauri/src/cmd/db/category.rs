use sqlx::QueryBuilder;
use tauri::State;

use models::Category;

use super::DbState;

#[tauri::command]
pub async fn add_category(db: State<'_, DbState>, category: Category) -> Result<(), String> {
    sqlx::query("INSERT INTO categories (tag, name) VALUES (?, ?)")
        .bind(&category.tag)
        .bind(&category.name)
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
