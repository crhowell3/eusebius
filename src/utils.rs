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

pub fn is_valid_date(s: &str) -> bool {
    if s.len() != 10 {
        return false;
    }

    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    if parts.get(0).map_or(false, |p| p.len() != 4)
        || parts.get(1).map_or(false, |p| p.len() != 2)
        || parts.get(2).map_or(false, |p| p.len() != 2)
    {
        return false;
    }

    let (Ok(y), Ok(m), Ok(d)) = (
        parts[0].parse::<i32>(),
        parts[1].parse::<u32>(),
        parts[2].parse::<u32>(),
    ) else {
        return false;
    };

    NaiveDate::from_ymd_opt(y, m, d).is_some()
}

pub fn format_date(raw: &str) -> String {
    let digits: String = raw.chars().filter(char::is_ascii_digit).collect();
    let digits = &digits[..digits.len().min(8)];

    match digits.len() {
        0 => String::new(),
        1..=4 => digits.to_string(),
        5..=6 => format!("{year}-{month}", year = &digits[..4], month = &digits[4..]),
        7..=8 => format!(
            "{year}-{month}-{day}",
            year = &digits[..4],
            month = &digits[4..6],
            day = &digits[6..]
        ),
        _ => unreachable!(),
    }
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

pub async fn fetch_records<T: DeserializeOwned>(cmd: &str) -> Result<Vec<T>, String> {
    let result = invoke(cmd, JsValue::UNDEFINED)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;
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
                .map(|m| m.matches())
                .unwrap_or(false);
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
