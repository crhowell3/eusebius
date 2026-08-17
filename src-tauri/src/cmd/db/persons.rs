use tauri::State;

use models::{MemberWorkView, Person};

use super::DbState;

#[tauri::command]
pub async fn get_persons_for_family(
    db: State<'_, DbState>,
    family_id: String,
) -> Result<Vec<Person>, String> {
    sqlx::query_as::<_, Person>(
        "SELECT id, family_id, role, first_name, last_name
        FROM persons
        WHERE family_id = ?
        ORDER BY CASE role
            WHEN 'head'   THEN 0
            WHEN 'spouse' THEN 1
            WHEN 'child'  THEN 2
        END",
    )
    .bind(&family_id)
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_works_for_person(
    db: State<'_, DbState>,
    person_id: i64,
) -> Result<Vec<MemberWorkView>, String> {
    sqlx::query_as::<_, MemberWorkView>(
        "SELECT mw.id, mw.person_id,
                p.first_name, p.last_name, p.role, p.family_id,
                w.id AS work_id, w.description,
                c.name AS category_name
         FROM member_works mw
         JOIN persons p ON mw.person_id = p.id
         JOIN works w ON mw.work_id = w.id
         JOIN categories c ON w.category_id = c.id
         WHERE mw.person_id = ?
         ORDER BY c.name, w.category_id",
    )
    .bind(person_id)
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_persons_for_work(
    db: State<'_, DbState>,
    work_id: i64,
) -> Result<Vec<MemberWorkView>, String> {
    sqlx::query_as::<_, MemberWorkView>(
        "SELECT mw.id, mw.person_id,
                p.first_name, p.last_name, p.role, p.family_id,
                w.id AS work_id, w.work_code, w.description,
                c.name AS category_name
         FROM member_works mw
         JOIN persons p ON mw.person_id = p.id
         JOIN works w ON mw.work_id = w.id
         JOIN categories c ON w.category_id = c.id
         WHERE mw.work_id = ?
         ORDER BY p.last_name, p.first_name",
    )
    .bind(work_id)
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_works_for_person(
    db: State<'_, DbState>,
    person_id: i64,
    work_ids: Vec<i64>,
) -> Result<(), String> {
    sqlx::query("DELETE FROM member_works WHERE person_id = ?")
        .bind(person_id)
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    for work_id in &work_ids {
        sqlx::query(
            "INSERT OR IGNORE INTO member_works (person_id, work_id) VALUES (?, ?)"
        )
        .bind(person_id)
        .bind(work_id)
        .execute(&db.0)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
