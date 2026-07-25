pub mod cmd;

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

            let pool = tauri::async_runtime::block_on(async {
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .after_connect(|conn, _| {
                        Box::pin(async move {
                            sqlx::query("PRAGMA foreign_keys = ON")
                                .execute(conn)
                                .await?;
                            Ok(())
                        })
                    })
                    .connect_with(
                        db_path
                            .parse::<sqlx::sqlite::SqliteConnectOptions>()
                            .unwrap()
                            .create_if_missing(true),
                    )
                    .await
                    .expect("failed to connect to database");

                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("failed to run database migrations");

                pool
            });

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
