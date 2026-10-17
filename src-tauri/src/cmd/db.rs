use sqlx::SqlitePool;
use tauri::Manager;

use models::TableInfo;

pub struct DbState(pub SqlitePool);

pub mod baptism;
pub mod category;
pub mod child;
pub mod death;
pub mod family;
pub mod persons;
pub mod spouse;
pub mod works;

pub use baptism::*;
pub use category::*;
pub use child::*;
pub use death::*;
pub use family::*;
pub use persons::*;
pub use spouse::*;
pub use works::*;

/// # Errors
///
/// Returns an error if the list of tables cannot be retrieved.
#[tauri::command]
pub async fn list_tables(app: tauri::AppHandle) -> Result<Vec<TableInfo>, String> {
    let pool = app
        .try_state::<DbState>()
        .ok_or("database is not initialized")?
        .0
        .clone();

    let path = pool
        .connect_options()
        .get_filename()
        .to_string_lossy()
        .to_string();

    let tables =
        sqlx::query_scalar!("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .fetch_all(&pool)
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
