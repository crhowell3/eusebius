use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;
use models::Category;

#[derive(Properties, PartialEq)]
pub struct CategoriesTableProps {
    pub refresh_trigger: u32,
}

async fn fetch_categories() -> Result<Vec<Category>, String> {
    let result = invoke("get_categories", JsValue::UNDEFINED).await;
    from_value::<Vec<Category>>(result).map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteCategoriesArgs {
    work_codes: Vec<String>,
}

async fn delete_categories(work_codes: Vec<String>) -> Result<(), String> {
    let args = to_value(&DeleteCategoriesArgs { work_codes }).map_err(|e| e.to_string())?;
    let result = invoke("delete_categories", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Clone, PartialEq)]
enum SortColumn {
    Tag,
    Name,
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
            column: SortColumn::Tag,
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

fn sort_categories(works: &[Category], sort: &SortState) -> Vec<Category> {
    let mut sorted = works.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::Tag => a.tag.cmp(&b.tag),
            SortColumn::Name => a.name.cmp(&b.name),
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

#[function_component(CategoriesTable)]
pub fn categories_table(props: &CategoriesTableProps) -> Html {
    let categories = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);

    {
        let categories = categories.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_categories().await {
                    Ok(data) => {
                        categories.set(data);
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

    let sorted_categories = sort_categories(&*categories, &*sort);

    let all_checked = !sorted_categories.is_empty()
        && sorted_categories
            .iter()
            .all(|w| (*selected).contains(&w.tag));
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let on_sort_tag = {
        let sort = sort.clone();
        Callback::from(move |_: MouseEvent| sort.set((*sort).clone().toggle(SortColumn::Tag)))
    };

    let on_sort_name = {
        let sort = sort.clone();
        Callback::from(move |_: MouseEvent| sort.set((*sort).clone().toggle(SortColumn::Name)))
    };

    let on_row_toggle = {
        let selected = selected.clone();
        Callback::from(move |tag: String| {
            let mut next = (*selected).clone();
            if next.contains(&tag) {
                next.remove(&tag);
            } else {
                next.insert(tag);
            }
            selected.set(next);
        })
    };

    let on_select_all = {
        let sorted_categories = sorted_categories.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if sorted_categories
                .iter()
                .all(|w| (*selected).contains(&w.tag))
            {
                selected.set(HashSet::new());
            } else {
                selected.set(sorted_categories.iter().map(|w| w.tag.clone()).collect());
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let categories = categories.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let works = categories.clone();
            spawn_local(async move {
                match delete_categories(to_delete).await {
                    Ok(_) => {
                        let remaining = (*works)
                            .iter()
                            .filter(|w| !(*selected).contains(&w.tag))
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
                    <h3 class="works-table-title">{ "Categories" }</h3>
                    if !sorted_categories.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_categories.len(),
                                if sorted_categories.len() == 1 { "" } else { "s" }) }
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
                } else if sorted_categories.is_empty() {
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
                        <span class="works-table-empty-sub">{ "Add a work code using the form" }</span>
                    </div>
                } else {
                    <table class="works-table">
                        <colgroup>
                            <col class="works-col-check" />
                            <col class="works-col-code" />
                            <col class="works-col-description" />
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
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_tag }>
                                    <span class="works-table-th-inner">
                                        { "Tag" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::Tag,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_name }>
                                    <span class="works-table-th-inner">
                                        { "Name" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::Name,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            { for sorted_categories.iter().map(|w| {
                                let tag = w.tag.clone();
                                let is_checked = (*selected).contains(&w.tag);
                                let on_row_toggle = on_row_toggle.clone();
                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ w.tag.clone() } class={ row_class }>
                                        <td class="works-table-td works-table-td--check">
                                            <input
                                                type="checkbox"
                                                checked={ is_checked }
                                                onchange={
                                                    Callback::from(move |_: Event| {
                                                        on_row_toggle.emit(tag.clone());
                                                    })
                                                }
                                            />
                                        </td>
                                        <td class="works-table-td">
                                            <span class="works-table-code-badge">
                                                { &w.tag }
                                            </span>
                                        </td>
                                        <td class="works-table-td">{ &w.name }</td>
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
