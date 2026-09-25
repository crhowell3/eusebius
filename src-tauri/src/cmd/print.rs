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

    for record in records {
        let first_name = &record.first_name;
        let last_name = &record.last_name;
        let day = &record.day;

        body.push_str(&format!(
            r#"
                <li>
                <span>{day}</span>
                <span>{first_name} {last_name}</span>
                </li>
           "#,
        ));
    }

    format!(
        r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8" />
        <title>Maintenance Records</title>
        <style>
            * {{ box-sizing: border-box; margin: 0; padding: 0; }}

            body {{
                font-family: "Helvetica Neue", Arial, sans-serif;
                font-size: 11px;
                color: #111;
                background: #fff;
                padding: 20px;
            }}

            .print-header {{
                display: flex;
                justify-content: space-between;
                align-items: baseline;
                border-bottom: 2px solid #111;
                padding-bottom: 8px;
                margin-bottom: 16px;
            }}

            .print-title {{
                font-size: 16px;
                font-weight: 700;
            }}

            .print-meta {{
                font-size: 10px;
                color: #555;
            }}

            .equipment-block {{
                margin-bottom: 5px;
                padding-bottom: 16px;
            }}

            .equipment-header {{
                display: flex;
                align-items: baseline;
                gap: 12px;
                margin-bottom: 8px;
                padding: 6px 8px;
                background: #f0f0f0;
                border-left: 4px solid #4a5fad;
            }}

            .equipment-id {{
                font-family: monospace;
                font-size: 10px;
                font-weight: 700;
                background: #4a5fad;
                color: #fff;
                padding: 1px 6px;
                border-radius: 3px;
            }}

            .equipment-name {{
                font-size: 13px;
                font-weight: 700;
            }}

            .equipment-meta {{
                font-size: 10px;
                color: #555;
                margin-left: auto;
            }}

            .info-table {{
                width: 100%;
                border-collapse: collapse;
                margin-bottom: 15px;
            }}

            .info-table td,
            .info-table th {{
                padding: 3px 8px;
                border: 1px solid #ddd;
                text-align: left;
                vertical-align: top;
            }}

            .section-label {{
                background: #e8ecf8;
                font-size: 10px;
                font-weight: 700;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                color: #3a4e96;
                padding: 4px 8px;
            }}

            .label {{
                font-weight: 600;
                color: #444;
                width: 90px;
                white-space: nowrap;
                background: #fafafa;
            }}

            .maintenance-records-header th {{
                background: #fafafa;
                font-weight: 600;
                font-size: 10px;
            }}

            .toolbar {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                background: #fff;
                border-bottom: 1px solid #ddd;
                padding: 8px 20px;
                display: flex;
                gap: 10px;
                align-items: center;
                z-index: 100;
                box-shadow: 0 1px 4px rgba(0,0,0,0.1);
            }}

            .toolbar button {{
                padding: 6px 16px;
                font-size: 12px;
                font-family: inherit;
                border-radius: 4px;
                cursor: pointer;
                border: 1px solid transparent;
            }}

            .btn-print {{
                background: #4a5fad;
                color: #fff;
                border-color: #3a4e96;
            }}

            .btn-print:hover {{ background: #3a4e96; }}

            .btn-close {{
                background: #f5f5f5;
                color: #333;
                border-color: #ccc;
            }}

            .btn-close:hover {{ background: #e8e8e8; }}

            .toolbar-info {{
                font-size: 11px;
                color: #777;
                margin-left: auto;
            }}

            /* Push content below fixed toolbar when not printing */
            .content {{ margin-top: 48px; }}

            @media print {{
                .toolbar {{ display: none; }}
                .content {{ margin-top: 0; }}
            }}
        </style>
    </head>
    <body>
        <div class="toolbar">
            <button class="btn-print" onclick="window.print()">&#128438; Print / Save as PDF</button>
            <button class="btn-close" onclick="window.close()">Close</button>
        </div>
        <div class="content">
            <div class="print-header">
                <span class="print-title">{month} Birthdays</span>
                <span class="print-meta">Printed: <script>document.write(new Date().toLocaleDateString())</script></span>
            </div>
            {}
        </div>
    </body>
    </html>"#,
        body
    )
}
