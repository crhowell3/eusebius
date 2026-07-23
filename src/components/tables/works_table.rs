use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;
use models::{Category, Work};

#[derive(Properties, PartialEq)]
pub struct WorksTableProps {
    pub refresh_trigger: u32,
}

async fn fetch_works() -> Result<Vec<Work>, String> {
    let result = invoke("get_works", JsValue::UNDEFINED).await;
    from_value::<Vec<Work>>(result).map_err(|e| e.to_string())
}

async fn delete_works(work_ids: Vec<i64>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        work_ids: Vec<i64>,
    }
    let args = to_value(&Args { work_ids }).map_err(|e| e.to_string())?;
    let result = invoke("delete_works", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Clone, PartialEq)]
enum SortColumn {
    Description,
}

#[derive(Clone, PartialEq)]
enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone, PartialEq)]
struct SortState {
    column: SortColumn,
    dir: SortDir,
}

impl SortState {
    fn default() -> Self {
        Self {
            column: SortColumn::Description,
            dir: SortDir::Asc,
        }
    }

    fn toggle(&self, col: SortColumn) -> Self {
        if self.column == col {
            Self {
                column: col,
                dir: match self.dir {
                    SortDir::Asc => SortDir::Desc,
                    SortDir::Desc => SortDir::Asc,
                },
            }
        } else {
            Self {
                column: col,
                dir: SortDir::Asc,
            }
        }
    }
}

fn sort_works(works: &[Work], sort: &SortState) -> Vec<Work> {
    let mut sorted = works.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::Description => a.description.cmp(&b.description),
        };
        match sort.dir {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });
    sorted
}

fn sort_icon(active: bool, dir: &SortDir) -> Html {
    if !active {
        return html! {
            <svg class="sort-icon sort-icon--inactive" xmlns="http://www.w3.org/2000/svg"
                width="12" height="12" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2"
                stroke-linecap="round" stroke-linejoin="round">
                <path d="M7 15l5 5 5-5"/>
                <path d="M7 9l5-5 5 5"/>
            </svg>
        };
    }
    match dir {
        SortDir::Asc => html! {
            <svg class="sort-icon sort-icon--active" xmlns="http://www.w3.org/2000/svg"
                width="12" height="12" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2.5"
                stroke-linecap="round" stroke-linejoin="round">
                <path d="M7 15l5 5 5-5"/>
            </svg>
        },
        SortDir::Desc => html! {
            <svg class="sort-icon sort-icon--active" xmlns="http://www.w3.org/2000/svg"
                width="12" height="12" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2.5"
                stroke-linecap="round" stroke-linejoin="round">
                <path d="M7 9l5-5 5 5"/>
            </svg>
        },
    }
}

#[function_component(WorksTable)]
pub fn works_table(props: &WorksTableProps) -> Html {
    let works = use_state(Vec::<Work>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);
    let categories = use_state(Vec::<Category>::new);

    {
        let categories = categories.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let result = invoke("get_categories", JsValue::UNDEFINED).await;
                if let Ok(data) = from_value::<Vec<Category>>(result) {
                    categories.set(data);
                }
            });
            || ()
        });
    }

    {
        let works = works.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
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
                initial_load.set(false);
            });
            || ()
        });
    }

    let sorted_works = sort_works(&*works, &*sort);

    let all_checked =
        !sorted_works.is_empty() && sorted_works.iter().all(|w| (*selected).contains(&w.id));
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let on_sort_description = {
        let sort = sort.clone();
        Callback::from(move |_: MouseEvent| {
            sort.set((*sort).clone().toggle(SortColumn::Description))
        })
    };

    let on_row_toggle = {
        let selected = selected.clone();
        Callback::from(move |id: i64| {
            let mut next = (*selected).clone();
            if next.contains(&id) {
                next.remove(&id);
            } else {
                next.insert(id);
            }
            selected.set(next);
        })
    };

    let on_select_all = {
        let sorted_works = sorted_works.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if sorted_works.iter().all(|w| (*selected).contains(&w.id)) {
                selected.set(HashSet::new());
            } else {
                selected.set(sorted_works.iter().map(|w| w.id.clone()).collect());
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let works = works.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let works = works.clone();
            spawn_local(async move {
                match delete_works(to_delete).await {
                    Ok(_) => {
                        let remaining = (*works)
                            .iter()
                            .filter(|w| !(*selected).contains(&w.id))
                            .cloned()
                            .collect();
                        works.set(remaining);
                        selected.set(HashSet::new());
                    }
                    Err(_e) => {
                        // TODO(@crhowell3): Add in proper error handling
                    }
                }
            });
        })
    };

    html! {
        <section class="works-table-card">
            <div class="works-table-toolbar">
                <div class="works-table-toolbar-left">
                    <h3 class="works-table-title">{ "Work Records" }</h3>
                    if !sorted_works.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_works.len(),
                                if sorted_works.len() == 1 { "" } else { "s" }) }
                        </span>
                    }
                </div>
                <div class="works-table-toolbar-right">
                    if some_checked {
                        <span class="works-table-selected-label">
                            { format!("{} selected", selected_count) }
                        </span>
                    }
                    <button
                        class="btn btn-icon danger"
                        disabled={ !some_checked }
                        onclick={ on_delete }
                        title="Delete selected"
                        aria-label="Delete selected"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="15" height="15"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="3 6 5 6 21 6" />
                            <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
                            <path d="M10 11v6" />
                            <path d="M14 11v6" />
                            <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" />
                        </svg>
                    </button>
                </div>
            </div>

            if let Some(err) = (*error).clone() {
                <div class="works-table-error">{ err }</div>
            }

            <div class="works-table-body">
                if *initial_load {
                    <div class="works-table-empty">
                        <span class="works-table-empty-text">{ "Loading..." }</span>
                    </div>
                } else if sorted_works.is_empty() {
                    <div class="works-table-empty">
                        <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
                            class="works-table-empty-icon">
                            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                            <polyline points="14 2 14 8 20 8"/>
                            <line x1="12" y1="18" x2="12" y2="12"/>
                            <line x1="9" y1="15" x2="15" y2="15"/>
                        </svg>
                        <span class="works-table-empty-text">{ "No records yet" }</span>
                        <span class="works-table-empty-sub">{ "Add a work record using the form" }</span>
                    </div>
                } else {
                    <table class="family-table">
                        <colgroup>
                            <col class="width: 44px" />
                            <col class="width: 120px" />
                            <col class="width: 300px" />
                        </colgroup>
                        <thead>
                            <tr>
                                <th class="works-table-th works-table-th--check">
                                    <input
                                        type="checkbox"
                                        checked={ all_checked }
                                        onchange={ on_select_all }
                                    />
                                </th>
                                <th class="works-table-th">{ "Category" }</th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_description }>
                                    <span class="works-table-th-inner">
                                        { "Description" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::Description,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            { for sorted_works.iter().map(|w| {
                                let id = w.id.clone();
                                let is_checked = (*selected).contains(&w.id);
                                let on_row_toggle = on_row_toggle.clone();
                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ w.id.clone() } class={ row_class }>
                                        <td class="works-table-td works-table-td--check">
                                            <input
                                                type="checkbox"
                                                checked={ is_checked }
                                                onchange={
                                                    Callback::from(move |_: Event| {
                                                        on_row_toggle.emit(id.clone());
                                                    })
                                                }
                                            />
                                        </td>
                                        <td class="works-table-td family-td-clip">
                                            { format!("[{}] {}", &w.category_tag, &w.category_name) }
                                        </td>
                                        <td class="works-table-td">{ &w.description }</td>
                                    </tr>
                                }
                            }) }
                        </tbody>
                    </table>
                }
            </div>
        </section>
    }
}
