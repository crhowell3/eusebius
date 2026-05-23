use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone)]
struct Family {
    family_id: String,
}

#[function_component(Members)]
pub fn members() -> Html {
    let _form_error = use_state(|| "".to_string());
    let new_family = use_state(|| Family {
        family_id: "".to_string(),
    });

    let handle_family_change = {
        let new_family = new_family.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            let mut updated = (*new_family).clone();

            match name.as_str() {
                "family_id" => updated.family_id = value,
                _ => {}
            }
            new_family.set(updated);
        })
    };

    html! {
        <div>
            <h3 style="margin-bottom: 1rem">{"Add Family"}</h3>

            <div class="form">
                <input
                    type="number"
                    name="family_id"
                    class="form-input"
                    placeholder=""
                    autoComplete="off"
                    min=1
                    step=1
                    value={(*new_family).clone().family_id}
                    onchange={handle_family_change.clone()}
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
                {"Add Family"}
            </button>
        </div>
    }
}
