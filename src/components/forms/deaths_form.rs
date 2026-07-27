use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::components::icons::{ErrorIcon, Plus};
use crate::utils::invoke;
use models::{Death, GenericAction};

#[derive(Serialize)]
struct AddDeathArgs {
    death: Death,
}

async fn add_death(death: Death) -> Result<(), String> {
    let args = to_value(&AddDeathArgs { death }).map_err(|e| e.to_string())?;
    let _ = invoke("add_death", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

#[derive(Properties, PartialEq)]
pub struct DeathsFormProps {
    pub on_death_added: Callback<()>,
}

#[function_component(DeathsForm)]
pub fn deaths_form(props: &DeathsFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_death = use_reducer(Death::default);

    let handle_death_change = {
        let new_death = new_death.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };
            new_death.dispatch(GenericAction::SetField { name, value });
        })
    };

    let handle_submit = {
        let new_death = new_death.clone();
        let form_error = form_error.clone();
        let on_death_added = props.on_death_added.clone();
        Callback::from(move |_: MouseEvent| {
            let death = (*new_death).clone();
            let form_error = form_error.clone();
            let new_death = new_death.dispatcher();
            let on_death_added = on_death_added.clone();
            spawn_local(async move {
                match add_death(death).await {
                    Ok(_) => {
                        new_death.dispatch(GenericAction::Reset);
                        form_error.set(None);
                        on_death_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            });
        })
    };

    let is_valid = !new_death.first_name.is_empty()
        && !new_death.last_name.is_empty()
        && !new_death.date_of_death.is_empty();

    html! {
        <aside class="works-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    <Plus />
                </span>
                <h3 class="works-form-title">{ "Add Death" }</h3>
            </div>

            <div class="works-form-body">
                if let Some(err) = (*form_error).as_deref() {
                    <div class="works-form-error">
                        <ErrorIcon />
                        { err }
                    </div>
                }

                <div class="member-form-section-label member-form-full">{ "Identity" }</div>

                <div class="form member-form-field">
                    <input type="text" name="first_name" class="form-input"
                        placeholder=" " autocomplete="off"
                        value={ new_death.first_name.clone() }
                        oninput={ handle_death_change.clone() } />
                    <label for="first_name" class="form-label">{ "First Name" }</label>
                </div>

                <div class="form member-form-field">
                    <input type="text" name="last_name" class="form-input"
                        placeholder=" " autocomplete="off"
                        value={ new_death.last_name.clone() }
                        oninput={ handle_death_change.clone() } />
                    <label for="last_name" class="form-label">{ "Last Name" }</label>
                </div>

                <div class="member-form-section-label member-form-full">{ "Date" }</div>

                <div class="form">
                    <input
                        type="text"
                        name="date_of_death"
                        autocomplete="off"
                        class="form-input"
                        placeholder=""
                        value={ new_death.date_of_death.clone() }
                        oninput={ handle_death_change.clone() }
                    />
                    <label for="date_of_death" class="form-label">
                        { "Date of Death" }
                    </label>
                </div>
            </div>

            <div class="works-form-footer">
                <button
                    class="btn btn-primary works-form-submit"
                    onclick={ handle_submit }
                    disabled={ !is_valid }
                >
                    { "Add Record" }
                </button>
            </div>
        </aside>
    }
}
