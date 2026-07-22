use sqlx::QueryBuilder;
use tauri::State;

use models::Child;

use super::DbState;

#[tauri::command]
pub async fn get_children_by_family(
    db: State<'_, DbState>,
    family_id: String,
) -> Result<Vec<Child>, String> {
    sqlx::query_as::<_, Child>(
        "SELECT id, family_id, first_name, last_name, is_member, is_active, date_of_birth, cell_phone, work_phone, email_address, on_bulletin_email_list
            FROM children
            WHERE family_id = ?
            ORDER BY id ASC"
        )
    .bind(&family_id)
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_children(
    db: State<'_, DbState>,
    family_id: String,
    children: Vec<Child>,
) -> Result<Vec<Child>, String> {
    for child in &children {
        if child.id <= 0 {
            sqlx::query(
                "INSERT INTO children
                    (family_id, first_name, last_name, is_member, is_active,
                     date_of_birth, cell_phone, work_phone, email_address, on_bulletin_email_list)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&child.family_id)
            .bind(&child.first_name)
            .bind(&child.last_name)
            .bind(child.is_member)
            .bind(child.is_active)
            .bind(&child.date_of_birth)
            .bind(&child.cell_phone)
            .bind(&child.work_phone)
            .bind(&child.email_address)
            .bind(child.on_bulletin_email_list)
            .execute(&db.0)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            sqlx::query(
                "UPDATE children SET
                    first_name = ?, last_name = ?, is_member = ?, is_active = ?,
                    date_of_birth = ?, cell_phone = ?, work_phone = ?,
                    email_address = ?, on_bulletin_email_list = ?
                 WHERE id = ?",
            )
            .bind(&child.first_name)
            .bind(&child.last_name)
            .bind(child.is_member)
            .bind(child.is_active)
            .bind(&child.date_of_birth)
            .bind(&child.cell_phone)
            .bind(&child.work_phone)
            .bind(&child.email_address)
            .bind(child.on_bulletin_email_list)
            .bind(child.id)
            .execute(&db.0)
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    sqlx::query_as::<_, Child>(
        "SELECT id, family_id, first_name, last_name, is_member, is_active,
                date_of_birth, cell_phone, work_phone, email_address, on_bulletin_email_list
         FROM children
         WHERE family_id = ?
         ORDER BY id ASC",
    )
    .bind(&family_id)
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_child(db: State<'_, DbState>, child: Child) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO children (family_id, first_name, last_name)
        VALUES (?, ?, ?)",
    )
    .bind(&child.family_id)
    .bind(&child.first_name)
    .bind(&child.last_name)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_children(db: State<'_, DbState>, child_ids: Vec<i64>) -> Result<(), String> {
    if child_ids.is_empty() {
        return Ok(());
    }
    let mut builder = QueryBuilder::new("DELETE FROM children WHERE id IN (");
    let mut separated = builder.separated(", ");
    for id in &child_ids {
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
