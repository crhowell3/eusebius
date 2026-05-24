pub mod cmd;
use crate::cmd::{add_work, get_works, DbState};
use tauri::Manager;

const WORKS_INIT: &'static str = "CREATE TABLE IF NOT EXISTS works (
    work_code       TEXT PRIMARY KEY NOT NULL,
    description     TEXT NOT NULL
)";

const BAPTISMS_INIT: &'static str = "CREATE TABLE IF NOT EXISTS baptisms (
    family_id       TEXT PRIMARY KEY NOT NULL,
    last_name       TEXT NOT NULL,
    first_name      TEXT NOT NULL,
    date_baptized   TEXT NOT NULL,
    witness         TEXT NOT NULL,
    location        TEXT NOT NULL
)";

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

                sqlx::query(WORKS_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize WORKS schema");

                sqlx::query(BAPTISMS_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize BAPTISMS schema");

                pool
            });

            app.manage(DbState(pool));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_works, add_work])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
