use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use shared::Spouse;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(Spouses)]
pub fn spouses() -> Html {
    let _form_error = use_state(|| String::new());
    let new_spouse = use_state(|| Spouse::new());

    let handle_spouse_change = {
        let new_spouse = new_spouse.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            let mut updated = (*new_spouse).clone();

            match name.as_str() {
                "family_id" => updated.family_id = value,
                _ => {}
            }
            new_spouse.set(updated);
        })
    };

    html! {
        <div>
            <hr style="margin: 1rem 0; width: 100%" />

            <h3 style="margin-bottom: 1rem">{"Add Spouse"}</h3>

            <div class="form">
                <input
                    type="number"
                    name="family_id"
                    class="form-input"
                    placeholder=""
                    autoComplete="off"
                    min=1
                    step=1
                    value={(*new_spouse).clone().family_id}
                    onchange={handle_spouse_change.clone()}
                />
                <label htmlFor="family_id" class="form-label">
                    {"Family ID"}
                </label>
            </div>

            <button
                class="btn btn-primary"
                //onclick={add_baptism}
                //disabled={new_baptism.family_id.is_empty() || new_baptism.last_name.is_empty()}
            >
                {"Add Spouse"}
            </button>
        </div>
    }
}
