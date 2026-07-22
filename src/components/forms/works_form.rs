use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::utils::invoke;
use models::{GenericAction, Work};

#[derive(Serialize)]
struct AddWorkArgs {
    work: Work,
}

async fn add_work(work: Work) -> Result<(), String> {
    let args = to_value(&AddWorkArgs { work }).map_err(|e| e.to_string())?;
    let result = invoke("add_work", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Properties, PartialEq)]
pub struct WorksFormProps {
    pub on_work_added: Callback<()>,
}

#[function_component(WorksForm)]
pub fn works_form(props: &WorksFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_work = use_reducer(Work::default);

    let handle_work_change = {
        let new_work = new_work.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };
            new_work.dispatch(GenericAction::SetField { name, value });
        })
    };

    let handle_submit = {
        let new_work = new_work.clone();
        let form_error = form_error.clone();
        let on_work_added = props.on_work_added.clone();
        Callback::from(move |_: MouseEvent| {
            let work = (*new_work).clone();
            let form_error = form_error.clone();
            let new_work = new_work.dispatcher();
            let on_work_added = on_work_added.clone();
            spawn_local(async move {
                match add_work(work).await {
                    Ok(_) => {
                        new_work.dispatch(GenericAction::Reset);
                        form_error.set(None);
                        on_work_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            });
        })
    };

    let is_valid = !new_work.work_code.is_empty() && !new_work.description.is_empty();

    html! {
        <aside class="works-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                        fill="none" stroke="currentColor" stroke-width="2.5"
                        stroke-linecap="round" stroke-linejoin="round">
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                </span>
                <h3 class="works-form-title">{ "New Work Code" }</h3>
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
                        type="text"
                        name="work_code"
                        autocomplete="off"
                        class="form-input"
                        placeholder=""
                        minlength="2"
                        maxlength="2"
                        value={ new_work.work_code.clone() }
                        oninput={ handle_work_change.clone() }
                    />
                    <label for="work_code" class="form-label">
                        { "Work Code" }
                    </label>
                </div>

                <div class="works-form-hint">{ "2-character identifier (e.g. \"A1\")" }</div>

                <div class="form">
                    <input
                        type="text"
                        name="description"
                        autocomplete="off"
                        class="form-input"
                        placeholder=""
                        value={ new_work.description.clone() }
                        oninput={ handle_work_change.clone() }
                    />
                    <label for="description" class="form-label">
                        { "Description" }
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
