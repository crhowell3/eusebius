use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn backup_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let backup_dir = app_dir.join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    Ok(backup_dir)
}

fn db_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(app_dir.join("eusebius.db"))
}

#[derive(serde::Serialize)]
pub struct BackupInfo {
    pub filename: String,
    pub created_at: String,
    pub size_bytes: u64,
}

#[tauri::command]
pub async fn list_backups(app: tauri::AppHandle) -> Result<Vec<BackupInfo>, String> {
    let dir = backup_dir(&app)?;

    let mut entries: Vec<BackupInfo> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".db") {
                return None;
            }
            let metadata = entry.metadata().ok()?;
            let size_bytes = metadata.len();

            let created_at = name
                .strip_prefix("backup_")
                .and_then(|s| s.strip_suffix(".db"))
                .map(|s| s.replace('_', " ").replace('-', ":").replacen(':', "-", 2))
                .unwrap_or_else(|| name.clone());

            Some(BackupInfo {
                filename: name,
                created_at,
                size_bytes,
            })
        })
        .collect();

    entries.sort_by(|a, b| b.filename.cmp(&a.filename));
    Ok(entries)
}

#[tauri::command]
pub async fn create_backup(app: tauri::AppHandle, max_backups: u32) -> Result<String, String> {
    let src = db_path(&app)?;
    let dir = backup_dir(&app)?;

    if !src.exists() {
        return Err("Database file not found".to_string());
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let filename = format!("backup_{}.db", format_timestamp(now));
    let dest = dir.join(&filename);

    fs::copy(&src, &dest).map_err(|e| format!("Failed to copy database: {}", e))?;

    let wal = src.with_extension("db-wal");
    let shm = src.with_extension("db-shm");
    if wal.exists() {
        fs::copy(&wal, dest.with_extension("db-wal")).ok();
    }
    if shm.exists() {
        fs::copy(&shm, dest.with_extension("db-shm")).ok();
    }

    prune_backups(&dir, max_backups)?;

    Ok(filename)
}

#[tauri::command]
pub async fn delete_backup(app: tauri::AppHandle, filename: String) -> Result<(), String> {
    if !filename.starts_with("backup_")
        || !filename.ends_with(".db")
        || filename.contains('/')
        || filename.contains('\\')
    {
        return Err("Invalid backup filename".to_string());
    }

    let dir = backup_dir(&app)?;
    let path = dir.join(&filename);

    if !path.exists() {
        return Err("Backup file not found".to_string());
    }

    fs::remove_file(&path).map_err(|e| e.to_string())?;
    Ok(())
}

fn prune_backups(dir: &PathBuf, max_backups: u32) -> Result<(), String> {
    let mut entries: Vec<String> = fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| {
            let e = e.ok()?;
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with("backup_") && name.ends_with(".db") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    entries.sort();

    while entries.len() > max_backups as usize {
        let oldest = entries.remove(0);
        let path = dir.join(&oldest);
        fs::remove_file(&path).ok();
        fs::remove_file(path.with_extension("db-wal")).ok();
        fs::remove_file(path.with_extension("db-shm")).ok();
    }

    Ok(())
}

fn format_timestamp(secs: u64) -> String {
    let s = secs;
    let sec = s % 60;
    let min = (s / 60) % 60;
    let hour = (s / 3600) % 24;
    let days = s / 86400;

    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        year, month, day, hour, min, sec
    )
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let mut d = days + 719468;
    let era = d / 146097;
    d %= 146097;
    let yoe = (d - d / 1460 + d / 36524 - d / 146096) / 365;
    let y = yoe + era * 400;
    let doy = d - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };
    (year, month, day)
}
