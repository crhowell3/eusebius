use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
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

fn has_db_extension(name: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("db"))
}

#[derive(serde::Serialize)]
pub struct BackupInfo {
    pub filename: String,
    pub created_at: String,
    pub size_bytes: u64,
}

/// # Errors
///
/// Returns an error if the backup directory does not exist or the list of backups cannot be read.
#[tauri::command]
pub async fn list_backups(app: tauri::AppHandle) -> Result<Vec<BackupInfo>, String> {
    let dir = backup_dir(&app)?;

    let mut entries: Vec<BackupInfo> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if !has_db_extension(&name) {
                return None;
            }
            let size_bytes = entry.metadata().ok()?.len();

            let created_at = name
                .strip_prefix("backup_")
                .and_then(|s| s.strip_suffix(".db"))
                .map_or_else(
                    || name.clone(),
                    |s| s.replace('_', " ").replace('-', ":").replacen(':', "-", 2),
                );

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

/// # Errors
///
/// Returns an error if the database file does not exist or the backup fails.
#[tauri::command]
pub async fn create_backup(app: tauri::AppHandle, max_backups: u32) -> Result<String, String> {
    let src = db_path(&app)?;
    let dir = backup_dir(&app)?;

    if !src.exists() {
        return Err("Database file not found".to_string());
    }

    let filename = format!("backup_{}.db", format_timestamp(Utc::now()));
    let dest = dir.join(&filename);

    fs::copy(&src, &dest).map_err(|e| format!("Failed to copy database: {e}"))?;

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

/// # Errors
///
/// Returns an error if the filename is invalid or the backup file does not exist.
#[tauri::command]
pub async fn delete_backup(app: tauri::AppHandle, filename: String) -> Result<(), String> {
    if !filename.starts_with("backup_")
        || !has_db_extension(&filename)
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
        .filter_map(|entry| Some(entry.ok()?.file_name().to_string_lossy().to_string()))
        .filter(|name| name.starts_with("backup_") && has_db_extension(name))
        .collect();

    entries.sort();

    let max_backups = usize::try_from(max_backups).unwrap_or(usize::MAX);
    let excess = entries.len().saturating_sub(max_backups);

    for oldest in entries.iter().take(excess) {
        let path = dir.join(oldest);
        fs::remove_file(&path).ok();
        fs::remove_file(path.with_extension("db-wal")).ok();
        fs::remove_file(path.with_extension("db-shm")).ok();
    }

    Ok(())
}

fn format_timestamp(time: DateTime<Utc>) -> String {
    time.format("%Y-%m-%d_%H-%M-%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp_epoch() {
        // Start of Unix Epoch time
        assert_eq!(
            format_timestamp(DateTime::UNIX_EPOCH),
            "1970-01-01_00-00-00"
        );
    }

    #[test]
    fn test_format_timestamp() {
        // Friday, July 14, 2017 at 2:40:00 AM UTC
        let formatted = DateTime::from_timestamp(1_500_000_000, 0).map(format_timestamp);

        assert_eq!(formatted.as_deref(), Some("2017-07-14_02-40-00"));
    }
}
