pub mod callback_factories;

use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use yew::prelude::{Html, html};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "clipboardManager"])]
    pub async fn writeText(text: &str);
}

#[must_use]
pub fn is_valid_date(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    let [y, m, d] = parts.as_slice() else {
        return false;
    };

    if y.len() != 4 || m.len() != 2 || d.len() != 2 {
        return false;
    }

    let (Ok(y), Ok(m), Ok(d)) = (y.parse::<i32>(), m.parse::<u32>(), d.parse::<u32>()) else {
        return false;
    };

    NaiveDate::from_ymd_opt(y, m, d).is_some()
}

pub fn format_date(raw: &str) -> String {
    let mut out = String::with_capacity(10);

    for (i, c) in raw.chars().filter(char::is_ascii_digit).take(8).enumerate() {
        if i == 4 || i == 6 {
            out.push('-');
        }
        out.push(c);
    }

    out
}

#[must_use]
pub fn format_phone(raw: &str) -> String {
    let mut out = String::with_capacity(14);

    for (i, c) in raw
        .chars()
        .filter(char::is_ascii_digit)
        .take(10)
        .enumerate()
    {
        match i {
            0 => out.push('('),
            3 => out.push_str(") "),
            6 => out.push('-'),
            _ => {}
        }
        out.push(c);
    }

    out
}

/// # Errors
///
/// Returns an error if the fetch fails or the response is not a valid JSON array.
pub async fn fetch_records<T: DeserializeOwned>(cmd: &str) -> Result<Vec<T>, String> {
    let result = invoke(cmd, JsValue::UNDEFINED)
        .await
        .map_err(|e| e.as_string().unwrap_or_else(|| "Unknown error".to_string()))?;
    from_value::<Vec<T>>(result).map_err(|e| e.to_string())
}

pub fn apply_theme(theme: &str) {
    let Some(win) = web_sys::window() else { return };
    let Some(doc) = win.document() else { return };
    let Some(root) = doc.document_element() else {
        return;
    };

    let resolved = match theme {
        "dark" => "dark",
        "light" => "light",
        _ => {
            let prefers_dark = win
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
                .is_some_and(|m| m.matches());
            if prefers_dark { "dark" } else { "light" }
        }
    };

    let _ = root.set_attribute("data-theme", resolved);
}

pub fn octoberware() -> Html {
    html! {
        <div>
            <text class="press-start-2p-regular">{ "OCTOBER" }
                <tspan style="color: #ef8354">{ "WARE" }</tspan>
            </text>
        </div>
    }
}
