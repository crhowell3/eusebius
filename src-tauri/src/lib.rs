pub mod cmd;

use crate::cmd::{DbState, backup, db, general, settings};

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
            app.manage(cmd::load_initial_settings(app.handle()));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            // Families
            db::family::add_family,
            db::family::get_families,
            db::family::delete_families,
            // Spouses
            db::spouse::save_spouse,
            db::spouse::get_spouse,
            // Children
            db::child::add_child,
            db::child::save_children,
            db::child::get_children_by_family,
            db::child::delete_children,
            // Baptisms
            db::baptism::add_baptism,
            db::baptism::get_baptisms,
            db::baptism::delete_baptisms,
            // Deaths
            db::death::add_death,
            db::death::get_deaths,
            db::death::delete_deaths,
            // Works
            db::works::add_work,
            db::works::get_works,
            db::works::update_work,
            db::works::delete_works,
            // Categories
            db::category::get_categories,
            db::category::add_category,
            db::category::update_category,
            db::category::delete_categories,
            // Settings
            settings::load_settings,
            settings::save_settings,
            settings::reset_settings,
            // Backups
            backup::list_backups,
            backup::create_backup,
            backup::delete_backup,
            // App / misc
            general::get_app_data_dir,
            general::exit_app,
            general::show_confirm_dialog,
            db::list_tables,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
