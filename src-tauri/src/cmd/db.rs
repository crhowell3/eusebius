use sqlx::SqlitePool;
use tauri::State;

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
