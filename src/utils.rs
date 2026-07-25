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
