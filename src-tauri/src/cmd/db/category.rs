use sqlx::QueryBuilder;
use tauri::State;

use models::Category;

use super::DbState;

fn next_tag(existing_tags: &[String]) -> String {
    let max = existing_tags
        .iter()
        .filter(|t| {
            !t.is_empty()
                && t.chars().all(|c| c.is_ascii_uppercase())
                && t.len() <= 26
                && t.as_str() != "MISC"
        })
        .max_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));

    match max {
        None => "A".to_string(),
        Some(tag) => increment_tag(tag),
    }
}

fn increment_tag(tag: &str) -> String {
    let mut chars: Vec<u8> = tag.bytes().collect();
    let mut i = chars.len() as i32 - 1;

    loop {
        if i < 0 {
            let mut result = vec![b'A'; chars.len() + 1];
            for c in result.iter_mut() {
                *c = b'A';
            }
            return String::from_utf8(result).unwrap();
        }

        if chars[i as usize] < b'Z' {
            chars[i as usize] += 1;
            return String::from_utf8(chars).unwrap();
        } else {
            chars[i as usize] = b'A';
            i -= 1;
        }
    }
}

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
pub async fn add_category(db: State<'_, DbState>, name: String) -> Result<Category, String> {
    if name.trim().is_empty() {
        return Err("Category name cannot be empty".to_string());
    }

    let existing_tags: Vec<String> = sqlx::query_scalar("SELECT tag FROM categories")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    let tag = next_tag(&existing_tags);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_tag() {
        let current_tag: Vec<String> = vec!["A".to_string()];
        let new_tag = next_tag(&current_tag);

        assert_eq!(new_tag, "B");
    }
}
