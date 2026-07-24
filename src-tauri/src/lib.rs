pub mod cmd;
mod schema;

use crate::cmd::db::*;
use crate::cmd::*;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_path = format!("sqlite:{}/eusebius.db", app_dir.display());

            let pool = tauri::async_runtime::block_on(init_database(&db_path))?;

            app.manage(DbState(pool));
            app.manage(load_initial_settings(app.handle()));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            // Families
            add_family,
            get_families,
            delete_families,
            // Spouses
            save_spouse,
            get_spouse,
            // Children
            add_child,
            save_children,
            get_children_by_family,
            delete_children,
            // Baptisms
            add_baptism,
            get_baptisms,
            delete_baptisms,
            // Deaths
            add_death,
            get_deaths,
            delete_deaths,
            // Works
            add_work,
            get_works,
            update_work,
            delete_works,
            // Categories
            get_categories,
            add_category,
            update_category,
            delete_categories,
            // Settings
            load_settings,
            save_settings,
            reset_settings,
            // Backups
            list_backups,
            create_backup,
            delete_backup,
            // App / misc
            get_app_data_dir,
            exit_app,
            show_confirm_dialog,
            list_tables,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn init_database(db_path: &str) -> Result<sqlx::SqlitePool, Box<dyn std::error::Error>> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            db_path
                .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true),
        )
        .await?;

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;

    for create_table in schema::ALL_TABLES {
        sqlx::query(*create_table).execute(&pool).await?;
    }

    setup_categories_table(&pool).await?;

    Ok(pool)
}
