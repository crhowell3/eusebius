use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::utils::invoke;
use shared::{Baptism, GenericAction};

#[derive(Serialize)]
struct AddBaptismArgs {
    baptism: Baptism,
}

async fn add_baptism(baptism: Baptism) -> Result<(), String> {
    let args = to_value(&AddBaptismArgs { baptism }).map_err(|e| e.to_string())?;
    let result = invoke("add_baptism", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Properties, PartialEq)]
pub struct BaptismsFormProps {
    pub on_baptism_added: Callback<()>,
}

#[function_component(BaptismsForm)]
pub fn baptisms_form(props: &BaptismsFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_baptism = use_reducer(Baptism::default);

    let handle_baptism_change = {
        let new_baptism = new_baptism.dispatcher();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            new_baptism.dispatch(GenericAction::SetField { name, value });
        })
    };

    let handle_submit = {
        let new_baptism = new_baptism.clone();
        let form_error = form_error.clone();
        let on_baptism_added = props.on_baptism_added.clone();
        Callback::from(move |_: MouseEvent| {
            let baptism = (*new_baptism).clone();
            let form_error = form_error.clone();
            let new_baptism = new_baptism.dispatcher();
            let on_baptism_added = on_baptism_added.clone();
            spawn_local(async move {
                match add_baptism(baptism).await {
                    Ok(_) => {
                        new_baptism.dispatch(GenericAction::Reset);
                        form_error.set(None);
                        on_baptism_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            })
        })
    };

    let is_valid = !new_baptism.family_id.is_empty();

    html! {
        <aside class="works-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    // Plus icon
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                        fill="none" stroke="currentColor" stroke-width="2.5"
                        stroke-linecap="round" stroke-linejoin="round">
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                </span>
                <h3 class="works-form-title">{ "New Baptism" }</h3>
            </div>

            <div class="works-form-body">
                if let Some(err) = (*form_error).as_deref() {
                    <div class="works-form-error">
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
            </div>

            <div class="works-form-footer">
                <button
                    class="btn btn-primary works-form-submit"
                    onclick={handle_submit}
                    disabled={ !is_valid }
                >
                    { "Add Baptism" }
                </button>
            </div>
        </aside>
    }
}
