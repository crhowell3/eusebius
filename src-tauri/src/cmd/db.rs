use sqlx::{QueryBuilder, SqlitePool};
use tauri::State;

use shared::{Baptism, Child, Death, Family, Spouse, TableInfo, Work};

pub struct DbState(pub SqlitePool);

//
// Generic Database Commands
//

#[tauri::command]
pub async fn list_tables(db: State<'_, DbState>) -> Result<Vec<TableInfo>, String> {
    let path =
        db.0.connect_options()
            .get_filename()
            .to_string_lossy()
            .to_string();

    let tables =
        sqlx::query_scalar!("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .fetch_all(&db.0)
            .await
            .map_err(|e| e.to_string())?;

    Ok(tables
        .into_iter()
        .flatten()
        .map(|name| TableInfo {
            name,
            path: path.clone(),
        })
        .collect())
}

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
// `Death` Commands
//

#[tauri::command]
pub async fn get_deaths(db: State<'_, DbState>) -> Result<Vec<Death>, String> {
    sqlx::query_as::<_, Death>("SELECT * FROM deaths")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_death(db: State<'_, DbState>, death: Death) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO deaths (first_name, last_name, date_of_death)
        VALUES (?, ?, ?)",
    )
    .bind(&death.first_name)
    .bind(&death.last_name)
    .bind(&death.date_of_death)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_deaths(db: State<'_, DbState>, death_ids: Vec<i64>) -> Result<(), String> {
    if death_ids.is_empty() {
        return Ok(());
    }
    let mut builder = QueryBuilder::new("DELETE FROM deaths WHERE id IN (");
    let mut separated = builder.separated(", ");
    for id in &death_ids {
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

//
// `Child` Commands
//

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

//
// `Spouse` Commands
//

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
    // INSERT OR REPLACE handles both create and update in one query
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

#[tauri::command]
pub async fn add_family(db: State<'_, DbState>, family: Family) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO families (family_id, mail_route, last_name, first_name, is_member, is_active, date_of_birth, anniversary_month, anniversary_day, home_phone, cell_phone, work_phone, address, city, state, zip, email_address, on_bulletin_email_list)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&family.family_id)
    .bind(&family.mail_route)
    .bind(&family.last_name)
    .bind(&family.first_name)
    .bind(&family.is_member)
    .bind(&family.is_active)
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
    .bind(&family.on_bulletin_email_list)
    .execute(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_families(
    db: State<'_, DbState>,
    family_ids: Vec<String>,
) -> Result<(), String> {
    if family_ids.is_empty() {
        return Ok(());
    }

    let mut builder = QueryBuilder::new("DELETE FROM families WHERE family_id IN (");

    let mut separated = builder.separated(", ");
    for id in &family_ids {
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
