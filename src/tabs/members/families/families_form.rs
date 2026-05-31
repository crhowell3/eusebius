use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::utils::invoke;

use shared::{Family, GenericAction};

#[derive(Serialize)]
struct AddFamilyArgs {
    family: Family,
}

async fn add_family(family: Family) -> Result<(), String> {
    let args = to_value(&AddFamilyArgs { family }).map_err(|e| e.to_string())?;
    let result = invoke("add_family", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Properties, PartialEq)]
pub struct FamiliesFormProps {
    pub on_family_added: Callback<()>,
}

#[function_component(FamiliesForm)]
pub fn families_form(props: &FamiliesFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_family = use_reducer(Family::default);

    let handle_family_change = {
        let new_family = new_family.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            new_family.dispatch(GenericAction::SetField { name, value });
        })
    };

    let handle_submit = {
        let new_family = new_family.clone();
        let form_error = form_error.clone();
        let on_family_added = props.on_family_added.clone();
        Callback::from(move |_: MouseEvent| {
            let family = (*new_family).clone();
            let form_error = form_error.clone();
            let new_family = new_family.dispatcher();
            let on_family_added = on_family_added.clone();
            spawn_local(async move {
                match add_family(family).await {
                    Ok(_) => {
                        new_family.dispatch(GenericAction::Reset);
                        form_error.set(None);
                        on_family_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            });
        })
    };

    let is_valid = !new_family.family_id.is_empty();

    html! {
        <div class="form-panel">
            <h3 class="panel-title">{ "Add Family" }</h3>

            if let Some(err) = (*form_error).clone() {
                <p class="text-error">{ err }</p>
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
                    value={new_family.family_id.clone()}
                    oninput={ handle_family_change.clone() }
                />
                <label htmlFor="family_id" class="form-label">
                    {"Family ID"}
                </label>
            </div>

            <button
                class="btn btn-primary"
                onclick={ handle_submit }
                disabled={ !is_valid }
            >
                { "Add Family" }
            </button>
        </div>
    }
}
