#![allow(clippy::needless_pass_by_value)]

use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

/// # Errors
///
/// Returns an error if the confirm dialog fails to open.
#[tauri::command]
pub fn exit_app(app: tauri::AppHandle) {
    app.dialog()
        .message("Are you sure you want to exit?")
        .title("Confirm Exit")
        .buttons(MessageDialogButtons::OkCancel)
        .show(move |confirmed| {
            if confirmed {
                app.exit(0);
            }
        });
}

/// # Errors
///
/// Returns an error if the confirm dialog fails to open.
#[tauri::command]
pub async fn show_confirm_dialog(app: tauri::AppHandle, title: String, message: String) -> bool {
    let (tx, rx) = std::sync::mpsc::channel();
    app.dialog()
        .message(message)
        .title(title)
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::OkCancel)
        .show(move |confirmed| {
            let _ = tx.send(confirmed);
        });
    rx.recv().unwrap_or(false)
}

/// # Errors
///
/// Returns an error if the app data directory cannot be retrieved.
#[tauri::command]
pub fn get_app_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    app.path()
        .app_data_dir()
        .map(|p| p.display().to_string())
        .map_err(|e| e.to_string())
}
