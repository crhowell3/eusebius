pub mod cmd;
use crate::cmd::*;
use tauri::Manager;

const DEATHS_INIT: &'static str = "CREATE TABLE IF NOT EXISTS deaths (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    date_of_death   TEXT NOT NULL
)";

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

const FAMILIES_INIT: &'static str = "CREATE TABLE IF NOT EXISTS families (
    family_id               TEXT PRIMARY KEY NOT NULL,
    mail_route              TEXT NOT NULL,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    is_member               INTEGER NOT NULL DEFAULT 0,
    is_active               INTEGER NOT NULL DEFAULT 0,
    date_of_birth           TEXT NOT NULL,
    anniversary_month       TEXT NOT NULL,
    anniversary_day         TEXT NOT NULL,
    home_phone              TEXT NOT NULL,
    cell_phone              TEXT NOT NULL,
    work_phone              TEXT NOT NULL,
    address                 TEXT NOT NULL,
    city                    TEXT NOT NULL,
    state                   TEXT NOT NULL,
    zip                     TEXT NOT NULL,
    email_address           TEXT NOT NULL,
    on_bulletin_email_list  INTEGER NOT NULL DEFAULT 0
)";

const SPOUSES_INIT: &'static str = "CREATE TABLE IF NOT EXISTS spouses (
    family_id               TEXT PRIMARY KEY NOT NULL,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    is_member               INTEGER NOT NULL DEFAULT 0,
    is_active               INTEGER NOT NULL DEFAULT 0,
    date_of_birth           TEXT,
    cell_phone              TEXT,
    work_phone              TEXT,
    email_address           TEXT,
    on_bulletin_email_list  INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (family_id) REFERENCES families(family_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

const CHILDREN_INIT: &'static str = "CREATE TABLE IF NOT EXISTS children (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id               TEXT NOT NULL,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    is_member               INTEGER NOT NULL DEFAULT 0,
    is_active               INTEGER NOT NULL DEFAULT 0,
    date_of_birth           TEXT,
    cell_phone              TEXT,
    work_phone              TEXT,
    email_address           TEXT,
    on_bulletin_email_list  INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (family_id) REFERENCES families(family_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
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

                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(&pool)
                    .await
                    .expect("failed to enable foreign keys");

                sqlx::query(DEATHS_INIT)
                    .execute(&pool)
                    .await
                    .expect("Failed to initialize DEATHS schema");

                sqlx::query(WORKS_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize WORKS schema");

                sqlx::query(BAPTISMS_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize BAPTISMS schema");

                sqlx::query(FAMILIES_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize FAMILIES schema");

                sqlx::query(SPOUSES_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize SPOUSES schema");

                sqlx::query(CHILDREN_INIT)
                    .execute(&pool)
                    .await
                    .expect("failed to initialize CHILDREN schema");

                pool
            });

            app.manage(DbState(pool));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            add_child,
            save_spouse,
            add_baptism,
            add_family,
            add_death,
            get_deaths,
            get_baptisms,
            delete_baptisms,
            delete_deaths,
            delete_families,
            delete_works,
            delete_children,
            get_children_by_family,
            get_families,
            get_spouse,
            get_works,
            add_work,
            exit_app,
            list_tables,
            save_children,
            list_backups,
            create_backup,
            delete_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
