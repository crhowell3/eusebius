use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

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
