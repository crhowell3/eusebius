use sqlx::QueryBuilder;
use tauri::State;

use models::Category;

use super::DbState;

pub async fn setup_categories_table(pool: &sqlx::SqlitePool) -> Result<(), String> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS categories (
            tag  TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "
        INSERT OR IGNORE INTO categories (tag, name) VALUES ('MISC', 'Miscellaneous')",
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_categories(db: State<'_, DbState>) -> Result<Vec<Category>, String> {
    sqlx::query_as::<_, Category>(
        "SELECT tag, name FROM categories ORDER BY
            CASE WHEN tag = 'MISC' THEN 1 ELSE 0 END,  -- MISC always last
            length(tag), tag",
    )
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_category(
    db: State<'_, DbState>,
    tag: String,
    name: String,
) -> Result<Category, String> {
    if tag.is_empty() || !tag.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Tag must contain only letters".to_string());
    }

    if name.trim().is_empty() {
        return Err("Category name cannot be empty".to_string());
    }

    let tag = tag.to_uppercase();

    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE tag = ?)")
        .bind(&tag)
        .fetch_one(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    if exists {
        return Err(format!("Tag \"{}\" is already in use", tag));
    }

    sqlx::query("INSERT INTO categories (tag, name) VALUES (?, ?)")
        .bind(&tag)
        .bind(name.trim())
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Category {
        tag,
        name: name.trim().to_string(),
    })
}

#[tauri::command]
pub async fn update_category(
    db: State<'_, DbState>,
    old_tag: String,
    new_tag: String,
    name: String,
) -> Result<(), String> {
    if new_tag.is_empty() || !new_tag.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Tag must contain only letters".to_string());
    }

    let new_tag = new_tag.to_uppercase();

    if new_tag != old_tag {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE tag = ?)")
                .bind(&new_tag)
                .fetch_one(&db.0)
                .await
                .map_err(|e| e.to_string())?;

        if exists {
            return Err(format!("Tag \"{}\" is already in use", new_tag));
        }

        sqlx::query("INSERT INTO categories (tag, name) VALUES (?, ?)")
            .bind(&new_tag)
            .bind(&name)
            .execute(&db.0)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("DELETE FROM categories WHERE tag = ?")
            .bind(&old_tag)
            .execute(&db.0)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        sqlx::query("UPDATE categories SET name = ? WHERE tag = ?")
            .bind(&name)
            .bind(&old_tag)
            .execute(&db.0)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_categories(db: State<'_, DbState>, tags: Vec<String>) -> Result<(), String> {
    if tags.is_empty() {
        return Ok(());
    }

    if tags.iter().any(|t| t == "MISC") {
        return Err("The Miscellaneous category cannot be deleted.".to_string());
    }

    for tag in &tags {
        sqlx::query("UPDATE works SET category_tag = 'MISC' WHERE category_tag = ?")
            .bind(tag)
            .execute(&db.0)
            .await
            .map_err(|e| e.to_string())?;
    }

    let mut builder = QueryBuilder::new("DELETE FROM categories WHERE tag IN (");
    let mut separated = builder.separated(", ");
    for tag in &tags {
        separated.push_bind(tag);
    }
    separated.push_unseparated(")");

    builder
        .build()
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
