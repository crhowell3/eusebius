use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Serialize, Deserialize)]
struct Work {
    work_code: String,
    description: String,
}

impl Default for Work {
    fn default() -> Self {
        Self {
            work_code: String::new(),
            description: String::new(),
        }
    }
}

#[derive(Serialize)]
struct AddWorkArgs {
    work: Work,
}

async fn fetch_works() -> Result<Vec<Work>, String> {
    let result = invoke("get_works", JsValue::UNDEFINED).await;
    from_value::<Vec<Work>>(result).map_err(|e| e.to_string())
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
struct WorkTableProps {
    pub refresh_trigger: u32,
}

#[function_component(WorkTable)]
fn work_table(props: &WorkTableProps) -> Html {
    let works = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let works = works.clone();
        let loading = loading.clone();
        let error = error.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            loading.set(true);
            spawn_local(async move {
                match fetch_works().await {
                    Ok(data) => {
                        works.set(data);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="table-panel">
            <h3 class="panel-title">{"Work Records"}</h3>

            if let Some(err) = (*error).clone() {
                <p class="text-error">{ err }</p>
            }

            if *loading {
                <p class="text-muted">{ "Loading..." }</p>
            } else if (*works).is_empty() {
                <p class="text-muted">{ "No records found." }</p>
            } else {
                <table class="results-table">
                    <thead>
                        <tr>
                            <th>{ "Work Code" }</th>
                            <th>{ "Description" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*works).iter().map(|w| html! {
                            <tr key={w.work_code.clone()}>
                                <td>{ &w.work_code }</td>
                                <td>{ &w.description }</td>
                            </tr>
                        })}
                    </tbody>
                </table>
            }
        </div>
    }
}

#[function_component(Works)]
pub fn works() -> Html {
    let form_error = use_state(|| None::<String>);
    let new_work = use_state(Work::default);
    let refresh_trigger = use_state(|| 0u32);

    let handle_work_change = {
        let new_work = new_work.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            let mut updated = (*new_work).clone();

            match name.as_str() {
                "work_code" => updated.work_code = value,
                "description" => updated.description = value,
                _ => {}
            }
            new_work.set(updated);
        })
    };

    let handle_submit = {
        let new_work = new_work.clone();
        let form_error = form_error.clone();
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: MouseEvent| {
            let work = (*new_work).clone();
            let form_error = form_error.clone();
            let new_work = new_work.clone();
            let refresh_trigger = refresh_trigger.clone();
            spawn_local(async move {
                match add_work(work).await {
                    Ok(_) => {
                        // Clear the form
                        new_work.set(Work::default());
                        form_error.set(None);
                        // Trigger table re-fetch
                        refresh_trigger.set(*refresh_trigger + 1);
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            });
        })
    };

    let is_valid = !new_work.work_code.is_empty() && !new_work.description.is_empty();

    html! {
        <div class="works-layout">
        // ── Left: Form ──────────────────────────────────────────────────
        <div class="form-panel">
            <h3 class="panel-title">{ "Add Work" }</h3>

            if let Some(err) = (*form_error).clone() {
                <p class="text-error">{ err }</p>
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
                    value={ (*new_work).work_code.clone() }
                    onchange={ handle_work_change.clone() }
                />
                <label for="work_code" class="form-label">
                    { "Work Code" }
                </label>
            </div>

            <div class="form">
                <input
                    type="text"
                    name="description"
                    autocomplete="off"
                    class="form-input"
                    placeholder=""
                    value={ (*new_work).description.clone() }
                    onchange={ handle_work_change.clone() }
                />
                <label for="description" class="form-label">
                    { "Description" }
                </label>
            </div>

            <button
                class="btn btn-primary"
                onclick={ handle_submit }
                disabled={ !is_valid }
            >
                { "Add Work" }
            </button>
        </div>

        // ── Right: Table ────────────────────────────────────────────────
        <WorkTable refresh_trigger={ *refresh_trigger } />
        </div>
    }
}
