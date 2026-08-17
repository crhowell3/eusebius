use tauri::State;

use models::Spouse;

use super::DbState;

#[tauri::command]
pub async fn get_spouse(
    db: State<'_, DbState>,
    family_id: String,
) -> Result<Option<Spouse>, String> {
    sqlx::query_as::<_, Spouse>(
        "SELECT family_id, first_name, last_name, date_of_birth,
                is_member, is_active, cell_phone, work_phone,
                email_address, on_bulletin_email_list
         FROM spouses WHERE family_id = ?",
    )
    .bind(&family_id)
    .fetch_optional(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_spouse(db: State<'_, DbState>, spouse: Spouse) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO spouses
            (family_id, first_name, last_name, date_of_birth,
             is_member, is_active, cell_phone, work_phone,
             email_address, on_bulletin_email_list)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(family_id) DO UPDATE SET
            first_name             = excluded.first_name,
            last_name              = excluded.last_name,
            date_of_birth          = excluded.date_of_birth,
            is_member              = excluded.is_member,
            is_active              = excluded.is_active,
            cell_phone             = excluded.cell_phone,
            work_phone             = excluded.work_phone,
            email_address          = excluded.email_address,
            on_bulletin_email_list = excluded.on_bulletin_email_list",
    )
    .bind(&spouse.family_id)
    .bind(&spouse.first_name)
    .bind(&spouse.last_name)
    .bind(&spouse.date_of_birth)
    .bind(spouse.is_member)
    .bind(spouse.is_active)
    .bind(&spouse.cell_phone)
    .bind(&spouse.work_phone)
    .bind(&spouse.email_address)
    .bind(spouse.on_bulletin_email_list)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO persons (family_id, role, first_name, last_name)
         VALUES (?, 'spouse', ?, ?)
         ON CONFLICT(family_id, role) WHERE role IN ('head', 'spouse') DO UPDATE SET
             first_name = excluded.first_name,
             last_name  = excluded.last_name",
    )
    .bind(&spouse.family_id)
    .bind(&spouse.first_name)
    .bind(&spouse.last_name)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
