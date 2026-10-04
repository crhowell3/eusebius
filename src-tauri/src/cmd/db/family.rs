use sqlx::QueryBuilder;
use tauri::Manager;

use models::Family;

use super::DbState;

/// # Errors
///
/// Returns an error if the families cannot be retrieved.
#[tauri::command]
pub async fn get_families(app: tauri::AppHandle) -> Result<Vec<Family>, String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query_as::<_, Family>("SELECT * FROM families")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
}

/// # Errors
///
/// Returns an error if the family cannot be added.
#[tauri::command]
pub async fn add_family(app: tauri::AppHandle, family: Family) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let next_id: String =
        sqlx::query_scalar::<_, Option<String>>("SELECT MAX(family_id) FROM families")
            .fetch_one(&pool)
            .await
            .map_err(|e| e.to_string())?
            .and_then(|max| max.parse::<u32>().ok())
            .map_or_else(
                || "0001".to_string(),
                |n| format!("{:04}", n.saturating_add(1)),
            );

    sqlx::query(
        "INSERT INTO families (family_id, mail_route, last_name, first_name, is_member, is_active, date_of_birth, anniversary_month, anniversary_day, home_phone, cell_phone, work_phone, address, city, state, zip, email_address, on_bulletin_email_list)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&next_id)
    .bind(&family.mail_route)
    .bind(&family.last_name)
    .bind(&family.first_name)
    .bind(family.is_member)
    .bind(family.is_active)
    .bind(&family.date_of_birth)
    .bind(&family.anniversary_month)
    .bind(&family.anniversary_day)
    .bind(&family.home_phone)
    .bind(&family.cell_phone)
    .bind(&family.work_phone)
    .bind(&family.address)
    .bind(&family.city)
    .bind(&family.state)
    .bind(&family.zip)
    .bind(&family.email_address)
    .bind(family.on_bulletin_email_list)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO persons (family_id, role, first_name, last_name)
         VALUES (?, 'head', ?, ?)",
    )
    .bind(&next_id)
    .bind(&family.first_name)
    .bind(&family.last_name)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the family cannot be updated.
#[tauri::command]
pub async fn update_family(app: tauri::AppHandle, family: Family) -> Result<(), String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    sqlx::query("UPDATE families SET mail_route = ?, last_name = ?, first_name = ?, is_member = ?, is_active = ?, date_of_birth = ?, anniversary_month = ?, anniversary_day = ?, home_phone = ?, cell_phone = ?, work_phone = ?, address = ?, city = ?, state = ?, zip = ?, email_address = ?, on_bulletin_email_list = ? WHERE family_id = ?")
        .bind(&family.mail_route)
        .bind(&family.last_name)
        .bind(&family.first_name)
        .bind(family.is_member)
        .bind(family.is_active)
        .bind(&family.date_of_birth)
        .bind(&family.anniversary_month)
        .bind(&family.anniversary_day)
        .bind(&family.home_phone)
        .bind(&family.cell_phone)
        .bind(&family.work_phone)
        .bind(&family.address)
        .bind(&family.city)
        .bind(&family.state)
        .bind(&family.zip)
        .bind(&family.email_address)
        .bind(family.on_bulletin_email_list)
        .bind(&family.family_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// # Errors
///
/// Returns an error if the families cannot be deleted.
#[tauri::command]
pub async fn delete_families(app: tauri::AppHandle, family_ids: Vec<String>) -> Result<(), String> {
    if family_ids.is_empty() {
        return Ok(());
    }

    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let mut builder = QueryBuilder::new("DELETE FROM families WHERE family_id IN (");

    let mut separated = builder.separated(", ");
    for id in &family_ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    builder
        .build()
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
