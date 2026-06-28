use std::fs;
use std::path::PathBuf;
use tauri::Manager;

use shared::AppSettings;

fn settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("settings.toml"))
}

#[tauri::command]
pub fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let path = settings_path(&app)?;

    if !path.exists() {
        return Ok(AppSettings::default());
    }

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read settings: {e}"))?;

    toml::from_str::<AppSettings>(&contents).map_err(|e| format!("Failed to parse settings: {e}"))
}

#[tauri::command]
pub fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let path = settings_path(&app)?;

    let contents = toml::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {e}"))?;

    fs::write(&path, contents).map_err(|e| format!("Failed to write settings: {e}"))?;

    Ok(())
}

#[tauri::command]
pub fn reset_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let defaults = AppSettings::default();
    save_settings(app, defaults.clone())?;
    Ok(defaults)
}
