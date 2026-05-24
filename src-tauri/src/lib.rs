use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::{Manager, State};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Work {
    work_code: String,
    description: String,
}

pub struct DbState(pub SqlitePool);

#[tauri::command]
async fn get_works(db: State<'_, DbState>) -> Result<Vec<Work>, String> {
    sqlx::query_as::<_, Work>("SELECT work_code, description FROM works")
        .fetch_all(&db.0)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app dir");
            let db_path = format!("sqlite:{}/wscoc.db", app_dir.display());

            let pool = tauri::async_runtime::block_on(async {
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .connect_with(
                        db_path
                            .parse::<sqlx::sqlite::SqliteConnectOptions>()
                            .unwrap()
                            .create_if_missing(true),
                    )
                    .await
                    .expect("failed to connect to database");

                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS works (

                    )",
                )
                .execute(&pool)
                .await
                .expect("failed to initialize database schema");
                SqlitePool::connect(&db_path)
                    .await
                    .expect("failed to connect to database")
            });

            app.manage(DbState(pool));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_works])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
