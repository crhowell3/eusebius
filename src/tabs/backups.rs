use serde::Deserialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;

#[derive(Clone, PartialEq, Deserialize)]
struct BackupInfo {
    filename: String,
    created_at: String,
    size_bytes: u64,
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

async fn fetch_backups() -> Result<Vec<BackupInfo>, String> {
    let result = invoke("list_backups", JsValue::UNDEFINED).await;
    from_value::<Vec<BackupInfo>>(result).map_err(|e| e.to_string())
}

async fn create_backup(max_backups: u32) -> Result<String, String> {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        max_backups: u32,
    }
    let args = to_value(&Args { max_backups }).map_err(|e| e.to_string())?;
    let result = invoke("create_backup", args).await;
    from_value::<String>(result).map_err(|e| e.to_string())
}

async fn delete_backup(filename: String) -> Result<(), String> {
    #[derive(serde::Serialize)]
    struct Args {
        filename: String,
    }
    let args = to_value(&Args { filename }).map_err(|e| e.to_string())?;
    let result = invoke("delete_backup", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[function_component(BackupsTabBody)]
pub fn backups_tab_body() -> Html {
    let backups = use_state(Vec::<BackupInfo>::new);
    let app_data_dir = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let creating = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let max_backups = use_state(|| 10u32);

    {
        let app_data_dir = app_data_dir.clone();
        let backups = backups.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let result = invoke("get_app_data_dir", JsValue::UNDEFINED).await;
                if let Ok(path) = from_value::<String>(result) {
                    app_data_dir.set(Some(path));
                }
                match fetch_backups().await {
                    Ok(data) => backups.set(data),
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_create = {
        let backups = backups.clone();
        let creating = creating.clone();
        let error = error.clone();
        let success = success.clone();
        let max_backups = max_backups.clone();
        Callback::from(move |_: MouseEvent| {
            let backups = backups.clone();
            let creating = creating.clone();
            let error = error.clone();
            let success = success.clone();
            let limit = *max_backups;
            creating.set(true);
            error.set(None);
            success.set(None);
            spawn_local(async move {
                match create_backup(limit).await {
                    Ok(filename) => {
                        success.set(Some(format!("Backup created: {}", filename)));
                        if let Ok(data) = fetch_backups().await {
                            backups.set(data);
                        }
                    }
                    Err(e) => error.set(Some(e)),
                }
                creating.set(false);
            });
        })
    };

    let on_delete = {
        let backups = backups.clone();
        let error = error.clone();
        let success = success.clone();
        Callback::from(move |filename: String| {
            let backups = backups.clone();
            let error = error.clone();
            let success = success.clone();
            let fname = filename.clone();
            spawn_local(async move {
                match delete_backup(fname.clone()).await {
                    Ok(_) => {
                        success.set(Some(format!("Deleted: {}", fname)));
                        if let Ok(data) = fetch_backups().await {
                            backups.set(data);
                        }
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    // TODO(@anyone): Move this to the settings page
    let on_max_change = {
        let max_backups = max_backups.clone();
        Callback::from(move |e: Event| {
            use wasm_bindgen::JsCast;
            use web_sys::HtmlInputElement;
            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<u32>() {
                    if val > 0 {
                        max_backups.set(val);
                    }
                }
            }
        })
    };

    html! {
        <div class="backup-tab-root">
            <div class="backup-header-card">
                <div class="backup-header">
                    <div class="backup-header-left">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
                            class="backup-header-icon">
                            <ellipse cx="12" cy="5" rx="9" ry="3"/>
                            <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                            <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                        </svg>
                        <div>
                            <h2 class="backup-title">{ "Database Backups" }</h2>
                            <p class="backup-subtitle">
                                { "Backups directory: " }
                                if let Some(path) = (*app_data_dir).as_deref() {
                                    <span>{ path.to_owned() + "/backups/" }</span>
                                }
                            </p>
                        </div>
                    </div>
                    <div class="backup-header-right">
                        <div class="backup-limit-control">
                            <label class="backup-limit-label" for="max_backups">
                                { "Keep last" }
                            </label>
                            <input
                                id="max_backups"
                                type="number"
                                class="backup-limit-input"
                                min="1"
                                max="100"
                                value={ (*max_backups).to_string() }
                                onchange={ on_max_change }
                            />
                            <span class="backup-limit-label">{ "backups" }</span>
                        </div>
                        <button
                            class="btn btn-primary"
                            onclick={ on_create }
                            disabled={ *creating }
                        >
                            if *creating {
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                                    viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                    class="backup-spin">
                                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                                </svg>
                                { " Creating..." }
                            } else {
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                                    viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                                    <polyline points="7 10 12 15 17 10"/>
                                    <line x1="12" y1="15" x2="12" y2="3"/>
                                </svg>
                                { " Create Backup" }
                            }
                        </button>
                    </div>
                </div>
                if let Some(err) = (*error).as_deref() {
                    <div class="backup-message backup-message--error">
                        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="10"/>
                            <line x1="12" y1="8" x2="12" y2="12"/>
                            <line x1="12" y1="16" x2="12.01" y2="16"/>
                        </svg>
                        { err }
                    </div>
                }

                if let Some(msg) = (*success).as_deref() {
                    <div class="backup-message backup-message--success">
                        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"/>
                        </svg>
                        { msg }
                    </div>
                }
            </div>
            <section class="works-table-card">
                <div class="works-table-toolbar">
                    <div class="works-table-toolbar-left">
                        <h3 class="works-table-title">{ "Existing Backups" }</h3>
                        if !(*backups).is_empty() {
                            <span class="works-table-count">
                                { format!("{} backup{}", (*backups).len(),
                                    if (*backups).len() == 1 { "" } else { "s" }) }
                            </span>
                        }
                    </div>
                </div>
                <div class="works-table-body">
                    if *loading {
                        <div class="works-table-empty">
                            <span class="works-table-empty-text">{ "Loading..." }</span>
                        </div>
                    } else if (*backups).is_empty() {
                        <div class="works-table-empty">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28"
                                viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
                                class="works-table-empty-icon">
                                <ellipse cx="12" cy="5" rx="9" ry="3"/>
                                <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                                <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                            </svg>
                            <span class="works-table-empty-text">{ "No backups yet" }</span>
                            <span class="works-table-empty-sub">
                                { "Click \"Create Backup\" to make your first backup" }
                            </span>
                        </div>
                    } else {
                        <table class="works-table">
                            <colgroup>
                                <col style="width: auto" />   // filename
                                <col style="width: 220px" />  // created at
                                <col style="width: 100px" />  // size
                                <col style="width: 60px" />   // delete
                            </colgroup>
                            <thead>
                                <tr>
                                    <th class="works-table-th">{ "Filename" }</th>
                                    <th class="works-table-th">{ "Created" }</th>
                                    <th class="works-table-th">{ "Size" }</th>
                                    <th class="works-table-th"></th>
                                </tr>
                            </thead>
                            <tbody>
                                { for (*backups).iter().map(|b| {
                                    let filename = b.filename.clone();
                                    let on_delete = on_delete.clone();
                                    html! {
                                        <tr key={ b.filename.clone() } class="works-table-row">
                                            <td class="works-table-td">
                                                <span class="backup-filename">
                                                    <svg xmlns="http://www.w3.org/2000/svg"
                                                        width="13" height="13"
                                                        viewBox="0 0 24 24" fill="none"
                                                        stroke="currentColor" stroke-width="2"
                                                        stroke-linecap="round" stroke-linejoin="round">
                                                        <ellipse cx="12" cy="5" rx="9" ry="3"/>
                                                        <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                                                        <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                                                    </svg>
                                                    { &b.filename }
                                                </span>
                                            </td>
                                            <td class="works-table-td">{ &b.created_at }</td>
                                            <td class="works-table-td">
                                                { format_size(b.size_bytes) }
                                            </td>
                                            <td class="works-table-td">
                                                <button
                                                    class="btn btn-icon danger btn-sm"
                                                    title="Delete backup"
                                                    onclick={ Callback::from(move |_: MouseEvent| {
                                                        on_delete.emit(filename.clone());
                                                    }) }
                                                >
                                                    <svg xmlns="http://www.w3.org/2000/svg"
                                                        width="13" height="13"
                                                        viewBox="0 0 24 24" fill="none"
                                                        stroke="currentColor" stroke-width="2"
                                                        stroke-linecap="round" stroke-linejoin="round">
                                                        <polyline points="3 6 5 6 21 6"/>
                                                        <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/>
                                                        <path d="M10 11v6"/>
                                                        <path d="M14 11v6"/>
                                                        <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2"/>
                                                    </svg>
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                }) }
                            </tbody>
                        </table>
                    }
                </div>
            </section>
        </div>
    }
}
