use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::components::icons::Plus;
use crate::utils::invoke;
use models::{Category, GenericAction, Work};

async fn add_work(work: Work) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        work: Work,
    }
    let args = to_value(&Args { work }).map_err(|e| e.to_string())?;
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
    pub refresh_trigger: u32,
}

#[function_component(WorksForm)]
pub fn works_form(props: &WorksFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_work = use_reducer(Work::default);
    let categories = use_state(Vec::<Category>::new);

    {
        let trigger = props.refresh_trigger;
        let categories = categories.clone();
        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                let result = invoke("get_categories", JsValue::UNDEFINED).await;
                if let Ok(data) = from_value::<Vec<Category>>(result) {
                    categories.set(data);
                }
            });
            || ()
        });
    }

    let handle_work_change = {
        let new_work = new_work.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                (select.name(), select.value())
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

    let is_valid = !new_work.description.trim().is_empty();

    html! {
        <aside class="works-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    <Plus />
                </span>
                <h3 class="works-form-title">{ "New Work" }</h3>
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
                    <select
                        name="category_tag"
                        class="form-input form-select"
                        oninput={ handle_work_change.clone() }
                    >
                        <option value="" disabled=true
                            selected={ new_work.category_tag.is_empty() }>
                            { "" }
                        </option>
                        { for (*categories).iter().map(|c| {
                            let selected = new_work.category_tag == c.tag;
                            html! {
                                <option value={ c.tag.clone() } selected={ selected }>
                                    { format!("[{}] {}", c.tag, c.name) }
                                </option>
                            }
                        }) }
                    </select>
                    <label for="category_tag" class="form-label">{ "Category" }</label>
                </div>

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
