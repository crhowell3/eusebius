use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use shared::Family;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn fetch_families() -> Result<Vec<Family>, String> {
    let result = invoke("get_families", JsValue::UNDEFINED).await;
    from_value::<Vec<Family>>(result).map_err(|e| e.to_string())
}

#[derive(Properties, PartialEq)]
struct FamiliesTableProps {
    pub refresh_trigger: u32,
}

#[function_component(FamiliesTable)]
fn families_table(props: &FamiliesTableProps) -> Html {
    let members = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let members = members.clone();
        let loading = loading.clone();
        let error = error.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            loading.set(true);
            spawn_local(async move {
                match fetch_families().await {
                    Ok(data) => {
                        members.set(data);
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
            <h3 class="panel-title">{"Family Records"}</h3>

            <button
                class="btn btn-icon danger"
                disabled=false
                //onClick={() => setShowConfirm(true)}
                title="Delete selected"
                aria-label="Delete selected"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="18"
                    height="18"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                >
                    <polyline points="3 6 5 6 21 6" />
                    <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
                    <path d="M10 11v6" />
                    <path d="M14 11v6" />
                    <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" />
                </svg>
            </button>

            if let Some(err) = (*error).clone() {
                <p class="text-error">{ err }</p>
            }

            if *loading {
                <p class="text-muted">{ "Loading..." }</p>
            } else if (*members).is_empty() {
                <p class="text-muted">{ "No records found." }</p>
            } else {
                <table class="results-table">
                    <thead>
                        <tr>
                            <th>{ "Family ID" }</th>
                            <th>{ "First Name" }</th>
                            <th>{ "Last Name" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*members).iter().map(|m| html! {
                            <tr key={m.family_id.clone()}>
                                <td>{ &m.family_id }</td>
                                <td>{ &m.first_name }</td>
                                <td>{ &m.last_name }</td>
                            </tr>
                        })}
                    </tbody>
                </table>
            }
        </div>
    }
}

#[function_component(Families)]
pub fn families() -> Html {
    let form_error = use_state(|| None::<String>);
    let new_family = use_state(|| Family::new());
    let refresh_trigger = use_state(|| 0u32);

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

    let handle_submit = {
        let new_family = new_family.clone();
        let form_error = form_error.clone();
        let refresh_trigger = refresh_trigger.clone();

        Callback::from(move |_: MouseEvent| {
            let family = (*new_family).clone();
            let form_error = form_error.clone();
            let new_family = new_family.clone();
            let refresh_trigger = refresh_trigger.clone();
            spawn_local(async move {
                // replace
            });
        })
    };

    let is_valid = !new_family.family_id.is_empty();

    html! {
        <div class="works-layout">
            <div class="form-panel">
                <h3 class="panel-title">{"Add Family"}</h3>

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
                        value={(*new_family).clone().family_id}
                        onchange={handle_family_change.clone()}
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
                    {"Add Family"}
                </button>
            </div>

            <FamiliesTable refresh_trigger={ *refresh_trigger } />
        </div>
    }
}
