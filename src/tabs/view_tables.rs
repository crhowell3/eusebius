use yew::prelude::*;

use crate::utils::invoke;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use shared::TableInfo;

#[function_component(ViewTables)]
pub fn view_tables() -> Html {
    let tables: UseStateHandle<Vec<TableInfo>> = use_state(Vec::new);
    let loading = use_state(|| true);
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    {
        let tables = tables.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let result = invoke("list_tables", JsValue::NULL).await;

                match from_value::<Vec<TableInfo>>(result) {
                    Ok(data) => tables.set(data),
                    Err(e) => error.set(Some(e.to_string())),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let mut groups: Vec<(String, Vec<String>)> = vec![];
    for t in (*tables).iter() {
        if let Some(group) = groups.iter_mut().find(|(p, _)| p == &t.path) {
            group.1.push(t.name.clone());
        } else {
            groups.push((t.path.clone(), vec![t.name.clone()]));
        }
    }

    html! {
        <div class="view-tables-wrapper">
            <div class="view-tables-card">

                <div class="view-tables-header">
                    <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                        class="view-tables-header-icon">
                        <ellipse cx="12" cy="5" rx="9" ry="3"/>
                        <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                        <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                    </svg>
                    <span class="view-tables-heading">{ "Tables" }</span>
                </div>

                <div class="view-tables-body">
                    if *loading {
                        <p class="view-tables-status">{ "Loading…" }</p>
                    } else if let Some(err) = (*error).clone() {
                        <p class="view-tables-status view-tables-status--error">{ err }</p>
                    } else {
                        <ul class="view-tables-list">
                            { for groups.iter().flat_map(|(path, names)| {
                                names.iter().map(move |name| html! {
                                    <li class="view-tables-row">
                                        <svg xmlns="http://www.w3.org/2000/svg"
                                            width="13" height="13" viewBox="0 0 24 24"
                                            fill="none" stroke="currentColor" stroke-width="2"
                                            stroke-linecap="round" stroke-linejoin="round"
                                            class="view-tables-row-icon"
                                            aria-hidden="true">
                                            <rect x="3" y="3" width="7" height="7"/>
                                            <rect x="14" y="3" width="7" height="7"/>
                                            <rect x="3" y="14" width="7" height="7"/>
                                            <rect x="14" y="14" width="7" height="7"/>
                                        </svg>
                                        <div class="view-tables-row-body">
                                            <span class="view-tables-row-name">{ name }</span>
                                            <span class="view-tables-row-path">{ path }</span>
                                        </div>
                                    </li>
                                })
                            }) }
                        </ul>
                    }
                </div>

            </div>
        </div>
    }
}
