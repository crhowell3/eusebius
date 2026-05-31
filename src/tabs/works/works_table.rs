use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;

use shared::Work;

#[derive(Properties, PartialEq)]
pub struct WorksTableProps {
    pub refresh_trigger: u32,
}

async fn fetch_works() -> Result<Vec<Work>, String> {
    let result = invoke("get_works", JsValue::UNDEFINED).await;
    from_value::<Vec<Work>>(result).map_err(|e| e.to_string())
}

#[function_component(WorksTable)]
pub fn works_table(props: &WorksTableProps) -> Html {
    let works = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    {
        let works = works.clone();
        let error = error.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_works().await {
                    Ok(data) => {
                        works.set(data);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    html! {
        <div class="table-panel">
            <h3 class="panel-title">{"Work Records"}</h3>

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

            if (*works).is_empty() {
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
