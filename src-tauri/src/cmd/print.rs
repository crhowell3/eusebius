use serde::Deserialize;
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintBirthdays {
    pub first_name: String,
    pub last_name: String,
    pub day: String,
}

#[tauri::command]
pub async fn print_birthdays(
    app: tauri::AppHandle,
    month: String,
    records: Vec<PrintBirthdays>,
) -> Result<(), String> {
    let html = generate_birthday_sheet(&month, &records);

    let tmp_path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("print_preview.html");

    std::fs::write(&tmp_path, html).map_err(|e| format!("Failed to write print file: {e}"))?;

    app.opener()
        .open_path(tmp_path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| format!("Failed to open print preview: {e}"))?;

    Ok(())
}

fn generate_birthday_sheet(month: &str, records: &[PrintBirthdays]) -> String {
    let mut body = String::new();
    let count = records.len();

    for record in records {
        let first_name = &record.first_name;
        let last_name = &record.last_name;
        let day = &record.day;

        body.push_str(&format!(
            r#"<div>
                <div class="header">
                    <span>{}</span>
                </div>
             </div>"#,
            month,
        ));
    }

    body
}
