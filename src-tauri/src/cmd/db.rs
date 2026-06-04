use sqlx::{QueryBuilder, SqlitePool};
use tauri::State;

use shared::{Baptism, Child, Family, Spouse, Work};

pub struct DbState(pub SqlitePool);

//
// `Baptism` Commands
//

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

//
// `Work` Commands
//

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

//
// `Child` Commands
//

#[tauri::command]
pub async fn get_children(db: State<'_, DbState>) -> Result<Vec<Child>, String> {
    sqlx::query_as::<_, Child>("SELECT family_id, first_name, last_name FROM children")
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

//
// `Spouse` Commands
//

#[tauri::command]
pub async fn get_spouse(db: State<'_, DbState>) -> Result<Vec<Spouse>, String> {
    sqlx::query_as::<_, Spouse>("SELECT family_id, first_name, last_name FROM spouses")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_spouse(db: State<'_, DbState>, spouse: Spouse) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO spouses (family_id, first_name, last_name)
        VALUES (?, ?, ?)",
    )
    .bind(&spouse.family_id)
    .bind(&spouse.first_name)
    .bind(&spouse.last_name)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

//
// `Family` Commands
//

#[tauri::command]
pub async fn get_families(db: State<'_, DbState>) -> Result<Vec<Family>, String> {
    sqlx::query_as::<_, Family>("SELECT * FROM families")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}
