use sqlx::QueryBuilder;
use tauri::State;

use models::Baptism;

use super::DbState;

#[tauri::command]
pub async fn get_baptisms(db: State<'_, DbState>) -> Result<Vec<Baptism>, String> {
    sqlx::query_as::<_, Baptism>("SELECT * FROM baptisms")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_baptism(db: State<'_, DbState>, baptism: Baptism) -> Result<(), String> {
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
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_baptism(db: State<'_, DbState>, baptism: Baptism) -> Result<(), String> {
    sqlx::query("UPDATE baptisms SET last_name = ?, first_name = ?, date_baptized = ?, witness = ?, location = ? WHERE family_id = ?")
        .bind(&baptism.last_name)
        .bind(&baptism.first_name)
        .bind(&baptism.date_baptized)
        .bind(&baptism.witness)
        .bind(&baptism.location)
        .bind(&baptism.family_id)
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_baptisms(
    db: State<'_, DbState>,
    family_ids: Vec<String>,
) -> Result<(), String> {
    if family_ids.is_empty() {
        return Ok(());
    }

    let mut builder = QueryBuilder::new("DELETE FROM baptisms WHERE family_id IN (");

    let mut separated = builder.separated(", ");
    for ids in &family_ids {
        separated.push_bind(ids);
    }
    separated.push_unseparated(")");

    builder
        .build()
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
