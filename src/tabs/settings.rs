use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::{apply_theme, invoke};
use shared::AppSettings;

async fn fetch_settings() -> Result<AppSettings, String> {
    let result = invoke("load_settings", JsValue::UNDEFINED).await;
    from_value::<AppSettings>(result).map_err(|e| e.to_string())
}

async fn persist_settings(settings: AppSettings) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        settings: AppSettings,
    }
    let args = to_value(&Args { settings }).map_err(|e| e.to_string())?;
    let result = invoke("save_settings", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

async fn do_reset() -> Result<AppSettings, String> {
    let result = invoke("reset_settings", JsValue::UNDEFINED).await;
    from_value::<AppSettings>(result).map_err(|e| e.to_string())
}

#[function_component(SettingsTabBody)]
pub fn settings_tab_body() -> Html {
    let settings = use_state(AppSettings::default);
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let dirty = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let show_reset_confirm = use_state(|| false);

    {
        let settings = settings.clone();
        let loading = loading.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_settings().await {
                    Ok(s) => settings.set(s),
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_save = {
        let settings = settings.clone();
        let saving = saving.clone();
        let dirty = dirty.clone();
        let error = error.clone();
        let success = success.clone();
        Callback::from(move |_: MouseEvent| {
            let payload = (*settings).clone();
            let saving = saving.clone();
            let dirty = dirty.clone();
            let error = error.clone();
            let success = success.clone();
            saving.set(true);
            error.set(None);
            success.set(None);
            spawn_local(async move {
                match persist_settings(payload).await {
                    Ok(_) => {
                        dirty.set(false);
                        success.set(Some("Settings saved.".to_string()));
                    }
                    Err(e) => error.set(Some(e)),
                }
                saving.set(false);
            });
        })
    };

    let on_confirm_reset = {
        let settings = settings.clone();
        let dirty = dirty.clone();
        let error = error.clone();
        let success = success.clone();
        let show_reset_confirm = show_reset_confirm.clone();
        Callback::from(move |_: MouseEvent| {
            let settings = settings.clone();
            let dirty = dirty.clone();
            let error = error.clone();
            let success = success.clone();
            let show_reset_confirm = show_reset_confirm.clone();
            spawn_local(async move {
                match do_reset().await {
                    Ok(defaults) => {
                        settings.set(defaults);
                        dirty.set(false);
                        success.set(Some("Settings reset to defaults.".to_string()));
                    }
                    Err(e) => error.set(Some(e)),
                }
                show_reset_confirm.set(false);
            });
        })
    };

    #[allow(unused_variables)]
    let set_string = |f: fn(&mut AppSettings, String)| {
        let settings = settings.clone();
        let dirty = dirty.clone();
        Callback::from(move |e: Event| {
            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                let mut next = (*settings).clone();
                f(&mut next, input.value());
                settings.set(next);
                dirty.set(true);
            }
        })
    };

    let set_u32 = |f: fn(&mut AppSettings, u32)| {
        let settings = settings.clone();
        let dirty = dirty.clone();
        Callback::from(move |e: Event| {
            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<u32>() {
                    let mut next = (*settings).clone();
                    f(&mut next, val);
                    settings.set(next);
                    dirty.set(true);
                }
            }
        })
    };

    let set_bool = |f: fn(&mut AppSettings, bool)| {
        let settings = settings.clone();
        let dirty = dirty.clone();
        Callback::from(move |e: Event| {
            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                let mut next = (*settings).clone();
                f(&mut next, input.checked());
                settings.set(next);
                dirty.set(true);
            }
        })
    };

    if *loading {
        return html! {
            <div class="settings-root">
                <div class="works-table-empty">
                    <span class="works-table-empty-text">{ "Loading settings..." }</span>
                </div>
            </div>
        };
    }

    html! {
        <div class="settings-root">
            <div class="settings-page-header">
                <div>
                    <h2 class="settings-page-title">{ "Settings" }</h2>
                    <p class="settings-page-subtitle">
                        { "Stored in settings.toml in your app data directory." }
                    </p>
                </div>
                <div class="settings-page-actions">
                    if *dirty {
                        <span class="works-table-selected-label">{ "Unsaved changes" }</span>
                    }
                    <button
                        class="btn btn-secondary btn-sm"
                        onclick={ Callback::from({
                            let show = show_reset_confirm.clone();
                            move |_: MouseEvent| show.set(true)
                        }) }
                    >
                        { "Reset to Defaults" }
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={ on_save }
                        disabled={ !*dirty || *saving }
                    >
                        { if *saving { "Saving..." } else { "Save Settings" } }
                    </button>
                </div>
            </div>

            if let Some(err) = (*error).as_deref() {
                <div class="settings-banner settings-banner--error">
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
                <div class="settings-banner settings-banner--success">
                    <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="20 6 9 17 4 12"/>
                    </svg>
                    { msg }
                </div>
            }

            if *show_reset_confirm {
                <div class="modal-backdrop" onclick={
                    Callback::from({ let s = show_reset_confirm.clone(); move |_: MouseEvent| s.set(false) })
                }>
                    <div class="modal" onclick={ Callback::from(|e: MouseEvent| e.stop_propagation()) }>
                        <h2 class="modal-title">{ "Reset to Defaults?" }</h2>
                        <p class="modal-message">
                            { "All settings will be restored to their default values. This cannot be undone." }
                        </p>
                        <div class="modal-actions">
                            <button class="btn btn-ghost" onclick={
                                Callback::from({ let s = show_reset_confirm.clone(); move |_: MouseEvent| s.set(false) })
                            }>
                                { "Cancel" }
                            </button>
                            <button class="btn btn-danger" onclick={ on_confirm_reset }>
                                { "Reset" }
                            </button>
                        </div>
                    </div>
                </div>
            }

            <div class="settings-groups">

                <div class="settings-group">
                    <div class="settings-group-header">
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="3"/>
                            <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
                        </svg>
                        <h3 class="settings-group-title">{ "Appearance" }</h3>
                    </div>

                    <div class="settings-rows">
                        <div class="settings-row">
                            <div class="settings-row-label">
                                <span class="settings-label">{ "Theme" }</span>
                                <span class="settings-description">
                                    { "Controls the color scheme of the application" }
                                </span>
                            </div>
                            <div class="settings-row-control">
                                <div class="settings-theme-picker">
                                    { for [("light", "Light"), ("dark", "Dark"), ("system", "System")].iter().map(|(val, label)| {
                                        let is_active = settings.theme == *val;
                                        let settings = settings.clone();
                                        let dirty = dirty.clone();
                                        let val_str = val.to_string();
                                        html! {
                                            <button
                                                class={ if is_active { "theme-option theme-option--active" } else { "theme-option" } }
                                                onclick={ Callback::from(move |_: MouseEvent| {
                                                    let mut next = (*settings).clone();
                                                    next.theme = val_str.clone();
                                                    settings.set(next);
                                                    dirty.set(true);
                                                    apply_theme(&val_str);
                                                }) }
                                            >
                                                { label }
                                            </button>
                                        }
                                    }) }
                                </div>
                            </div>
                        </div>

                        <div class="settings-row">
                            <div class="settings-row-label">
                                <span class="settings-label">{ "Font Size" }</span>
                                <span class="settings-description">
                                    { "Base font size in pixels (default: 15)" }
                                </span>
                            </div>
                            <div class="settings-row-control">
                                <input
                                    type="number"
                                    class="settings-number-input"
                                    min="11"
                                    max="22"
                                    value={ settings.font_size.to_string() }
                                    onchange={ set_u32(|s, v| s.font_size = v) }
                                />
                                <span class="settings-unit">{ "px" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="settings-group">
                    <div class="settings-group-header">
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <ellipse cx="12" cy="5" rx="9" ry="3"/>
                            <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                            <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                        </svg>
                        <h3 class="settings-group-title">{ "Data" }</h3>
                    </div>

                    <div class="settings-rows">
                        <div class="settings-row">
                            <div class="settings-row-label">
                                <span class="settings-label">{ "Confirm Before Delete" }</span>
                                <span class="settings-description">
                                    { "Show a confirmation dialog before deleting records" }
                                </span>
                            </div>
                            <div class="settings-row-control">
                                <label class="settings-toggle">
                                    <input
                                        type="checkbox"
                                        class="settings-toggle-input"
                                        checked={ settings.confirm_before_delete }
                                        onchange={ set_bool(|s, v| s.confirm_before_delete = v) }
                                    />
                                    <span class="settings-toggle-track">
                                        <span class="settings-toggle-thumb" />
                                    </span>
                                </label>
                            </div>
                        </div>

                        <div class="settings-row">
                            <div class="settings-row-label">
                                <span class="settings-label">{ "Rows Per Page" }</span>
                                <span class="settings-description">
                                    { "Maximum number of records shown in tables" }
                                </span>
                            </div>
                            <div class="settings-row-control">
                                <input
                                    type="number"
                                    class="settings-number-input"
                                    min="10"
                                    max="500"
                                    value={ settings.rows_per_page.to_string() }
                                    onchange={ set_u32(|s, v| s.rows_per_page = v) }
                                />
                                <span class="settings-unit">{ "rows" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="settings-group">
                    <div class="settings-group-header">
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                            <polyline points="7 10 12 15 17 10"/>
                            <line x1="12" y1="15" x2="12" y2="3"/>
                        </svg>
                        <h3 class="settings-group-title">{ "Backups" }</h3>
                    </div>

                    <div class="settings-rows">
                        <div class="settings-row">
                            <div class="settings-row-label">
                                <span class="settings-label">{ "Maximum Backups" }</span>
                                <span class="settings-description">
                                    { "Oldest backups are deleted once this limit is reached" }
                                </span>
                            </div>
                            <div class="settings-row-control">
                                <input
                                    type="number"
                                    class="settings-number-input"
                                    min="1"
                                    max="100"
                                    value={ settings.max_backups.to_string() }
                                    onchange={ set_u32(|s, v| s.max_backups = v) }
                                />
                                <span class="settings-unit">{ "backups" }</span>
                            </div>
                        </div>

                        <div class="settings-row">
                            <div class="settings-row-label">
                                <span class="settings-label">{ "Backup on Startup" }</span>
                                <span class="settings-description">
                                    { "Automatically create a backup each time the app launches" }
                                </span>
                            </div>
                            <div class="settings-row-control">
                                <label class="settings-toggle">
                                    <input
                                        type="checkbox"
                                        class="settings-toggle-input"
                                        checked={ settings.backup_on_startup }
                                        onchange={ set_bool(|s, v| s.backup_on_startup = v) }
                                    />
                                    <span class="settings-toggle-track">
                                        <span class="settings-toggle-thumb" />
                                    </span>
                                </label>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
