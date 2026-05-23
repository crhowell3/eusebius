use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone)]
struct Baptism {
    family_id: String,
    last_name: String,
    first_name: String,
    date_baptized: String,
    witness: String,
    location: String,
}

#[function_component(Baptisms)]
pub fn baptisms() -> Html {
    //  TODO(@cameron): actually use form_error
    let _form_error = use_state(|| "".to_string());
    let new_baptism = use_state(|| Baptism {
        family_id: "".to_string(),
        last_name: "".to_string(),
        first_name: "".to_string(),
        date_baptized: "".to_string(),
        witness: "".to_string(),
        location: "".to_string(),
    });

    let handle_baptism_change = {
        let new_baptism = new_baptism.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            let mut updated = (*new_baptism).clone();

            match name.as_str() {
                "family_id" => updated.family_id = value,
                "last_name" => updated.last_name = value,
                "first_name" => updated.first_name = value,
                "date_baptized" => updated.date_baptized = value,
                "witness" => updated.witness = value,
                "location" => updated.location = value,
                _ => {
                    unreachable!()
                }
            }
            new_baptism.set(updated);
        })
    };

    //  TODO(@cameron): implement this
    let add_baptism = { Callback::from(move |_| unimplemented!()) };

    html! {
        <div>
            <h3 style="margin-bottom: 1rem">{"Add Baptism"}</h3>

            <div class="form">
                <input
                    type="number"
                    name="family_id"
                    class="form-input"
                    placeholder=""
                    autoComplete="off"
                    min=1
                    step=1
                    value={(*new_baptism).clone().family_id}
                    onchange={handle_baptism_change.clone()}
                />
                <label htmlFor="family_id" class="form-label">
                    {"Family ID"}
                </label>
            </div>

            <div style="">
                <div class="form">
                    <input
                        type="text"
                        name="first_name"
                        autoComplete="off"
                        class="form-input"
                        placeholder=""
                        value={(*new_baptism).clone().first_name}
                        onchange={handle_baptism_change.clone()}
                    />
                    <label htmlFor="first_name" class="form-label">
                        {"First Name"}
                    </label>
                </div>

                <div class="form">
                    <input
                        type="text"
                        name="last_name"
                        autoComplete="off"
                        class="form-input"
                        placeholder=""
                        value={(*new_baptism).clone().last_name}
                        onchange={handle_baptism_change.clone()}
                    />
                    <label htmlFor="last_name" class="form-label">
                        {"Last Name"}
                    </label>
                </div>
            </div>

            <div class="form">
                <input
                    type="date"
                    name="date_baptized"
                    class="form-input"
                    placeholder=""
                    value={(*new_baptism).clone().date_baptized}
                    onchange={handle_baptism_change.clone()}
                />
                <label htmlFor="date_baptized" class="form-label">
                    {"Date"}
                </label>
            </div>

            <div class="form">
                <input
                    type="text"
                    name="witness"
                    class="form-input"
                    placeholder=""
                    value={(*new_baptism).clone().witness}
                    onchange={handle_baptism_change.clone()}
                />
                <label htmlFor="witness" class="form-label">
                    {"Witness"}
                </label>
            </div>

            <div class="form">
                <input
                    type="text"
                    name="location"
                    class="form-input"
                    placeholder=""
                    value={(*new_baptism).clone().location}
                    onchange={handle_baptism_change.clone()}
                />
                <label htmlFor="location" class="form-label">
                    {"Location"}
                </label>
            </div>

            <button
                class="btn btn-primary"
                onclick={add_baptism}
                disabled={new_baptism.family_id.is_empty() || new_baptism.last_name.is_empty()}
            >
                {"Add Baptism"}
            </button>
        </div>
    }
}
