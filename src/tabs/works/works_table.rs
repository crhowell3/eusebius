use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteWorksArgs {
    work_codes: Vec<String>,
}

async fn delete_works(work_codes: Vec<String>) -> Result<(), String> {
    let args = to_value(&DeleteWorksArgs { work_codes }).map_err(|e| e.to_string())?;
    let result = invoke("delete_works", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[function_component(WorksTable)]
pub fn works_table(props: &WorksTableProps) -> Html {
    let works = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);

    {
        let works = works.clone();
        let error = error.clone();
        let selected = selected.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_works().await {
                    Ok(data) => {
                        works.set(data);
                        error.set(None);
                        selected.set(HashSet::new());
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let all_checked =
        !(*works).is_empty() && (*works).iter().all(|w| (*selected).contains(&w.work_code));
    let some_checked = !(*selected).is_empty();

    let on_row_toggle = {
        let selected = selected.clone();
        Callback::from(move |work_code: String| {
            let mut next = (*selected).clone();
            if next.contains(&work_code) {
                next.remove(&work_code);
            } else {
                next.insert(work_code);
            }
            selected.set(next);
        })
    };

    let on_select_all = {
        let works = works.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if (*works).iter().all(|w| (*selected).contains(&w.work_code)) {
                selected.set(HashSet::new());
            } else {
                selected.set((*works).iter().map(|w| w.work_code.clone()).collect());
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let works = works.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let works = works.clone();
            spawn_local(async move {
                match delete_works(to_delete).await {
                    Ok(_) => {
                        let remaining = (*works)
                            .iter()
                            .filter(|w| !(*selected).contains(&w.work_code))
                            .cloned()
                            .collect();
                        works.set(remaining);
                        selected.set(HashSet::new());
                    }
                    Err(e) => {}
                }
            });
        })
    };

    html! {
        <div class="table-panel">
            <h3 class="panel-title">{"Work Records"}</h3>

            <button
                class="btn btn-icon danger"
                disabled={ !some_checked }
                onclick={ on_delete }
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
                        <th>
                            <input
                                type="checkbox"
                                checked={ all_checked }
                                onchange={ on_select_all }
                            />
                        </th>
                            <th>{ "Work Code" }</th>
                            <th>{ "Description" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*works).iter().map(|w| {
                            let work_code = w.work_code.clone();
                            let is_checked = (*selected).contains(&w.work_code);
                            let on_row_toggle = on_row_toggle.clone();

                            html! {
                                <tr key={w.work_code.clone()}>
                                    <td>
                                        <input
                                            type="checkbox"
                                            checked={ is_checked }
                                            onchange={
                                                Callback::from(move |_: Event| {
                                                    on_row_toggle.emit(work_code.clone());
                                                })
                                            }
                                        />
                                    </td>
                                    <td>{ &w.work_code }</td>
                                    <td>{ &w.description }</td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            }
        </div>
    }
}
