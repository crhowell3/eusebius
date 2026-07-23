use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::icons::{EditBox, Eye, Save, TrashCan};
use crate::components::sorting::{SortDir, SortState, sort_icon};
use crate::on_sort;
use crate::utils::{fetch_records, invoke};
use models::{Category, Work};

#[derive(Properties, PartialEq)]
pub struct WorksTableProps {
    pub refresh_trigger: u32,
}

#[derive(Clone, PartialEq)]
enum TableMode {
    View,
    Edit,
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

async fn update_work(work: Work) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        work: Work,
    }
    let args = to_value(&Args { work }).map_err(|e| e.to_string())?;
    let result = invoke("update_work", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown_error".to_string()))
    }
}

#[derive(Default, Clone, PartialEq)]
enum SortColumn {
    #[default]
    Category,
    Description,
}

fn sort_works(works: &[Work], sort: &SortState<SortColumn>) -> Vec<Work> {
    let mut sorted = works.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::Category => a.category_tag.cmp(&b.category_tag),
            SortColumn::Description => a.description.cmp(&b.description),
        };
        match sort.dir {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });
    sorted
}

#[function_component(WorksTable)]
pub fn works_table(props: &WorksTableProps) -> Html {
    let works = use_state(Vec::<Work>::new);
    let saved_works = use_state(Vec::<Work>::new);
    let categories = use_state(Vec::<Category>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);

    {
        let categories = categories.clone();
        let error = error.clone();
        let trigger = props.refresh_trigger;
        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Category>("get_categories").await {
                    Ok(data) => {
                        categories.set(data);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    {
        let works = works.clone();
        let saved_works = saved_works.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Work>("get_works").await {
                    Ok(data) => {
                        saved_works.set(data.clone());
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
    let dirty = *works != *saved_works;
    let all_checked =
        !sorted_works.is_empty() && sorted_works.iter().all(|w| (*selected).contains(&w.id));
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let on_sort_category = on_sort!(SortColumn::Category, sort);
    let on_sort_description = on_sort!(SortColumn::Description, sort);

    let on_toggle_mode = {
        let mode = mode.clone();
        let works = works.clone();
        let saved_works = saved_works.clone();
        let selected = selected.clone();

        Callback::from(move |_: MouseEvent| {
            if *mode == TableMode::Edit {
                works.set((*saved_works).clone());
                selected.set(HashSet::new());
                mode.set(TableMode::View);
            } else {
                mode.set(TableMode::Edit);
            }
        })
    };

    let on_save_edits = {
        let works = works.clone();
        let saved_works = saved_works.clone();
        let error = error.clone();
        let saving = saving.clone();
        let mode = mode.clone();
        let saved_snapshot = (*saved_works).clone();

        Callback::from(move |_: MouseEvent| {
            let changed: Vec<Work> = (*works)
                .iter()
                .filter(|w| {
                    !saved_snapshot.iter().any(|s| {
                        s.id == w.id
                            && s.description == w.description
                            && s.category_tag == w.category_tag
                    })
                })
                .cloned()
                .collect();

            if changed.is_empty() {
                mode.set(TableMode::View);
                return;
            }

            let works = works.clone();
            let saved_works = saved_works.clone();
            let error = error.clone();
            let saving = saving.clone();
            let mode = mode.clone();
            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for work in changed {
                    if let Err(e) = update_work(work).await {
                        error.set(Some(e));
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    saved_works.set((*works).clone());
                    error.set(None);
                    mode.set(TableMode::View);
                }
                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let works = works.clone();
        let categories = categories.clone();

        Callback::from(move |(id, field, value): (i64, &'static str, String)| {
            works.set({
                let mut next = (*works).clone();
                if let Some(w) = next.iter_mut().find(|w| w.id == id) {
                    match field {
                        "description" => w.description = value,
                        "category_tag" => {
                            let name = (*categories)
                                .iter()
                                .find(|c| c.tag == value)
                                .map(|c| c.name.clone())
                                .unwrap_or_default();
                            w.category_tag = value;
                            w.category_name = name;
                        }
                        _ => {}
                    }
                }
                next
            });
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
        let saved_works = saved_works.clone();
        let error = error.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let count = to_delete.len();
            let selected = selected.clone();
            let works = works.clone();
            let saved_works = saved_works.clone();
            let error = error.clone();
            let mode = mode.clone();

            spawn_local(async move {
                #[derive(Serialize)]
                struct DialogArgs {
                    message: String,
                    title: String,
                }
                let args = to_value(&DialogArgs {
                    message: format!(
                        "Delete {} selected work{}? This cannot be undone.",
                        count,
                        if count == 1 { "" } else { "s" }
                    ),
                    title: "Confirm Delete".to_string(),
                })
                .unwrap();

                let confirmed = invoke("show_confirm_dialog", args).await;
                if !from_value::<bool>(confirmed).unwrap_or(false) {
                    return;
                }

                match delete_works(to_delete).await {
                    Ok(_) => {
                        let remaining: Vec<Work> = (*works)
                            .iter()
                            .filter(|w| !(*selected).contains(&w.id))
                            .cloned()
                            .collect();
                        works.set(remaining.clone());
                        saved_works.set(remaining);
                        selected.set(HashSet::new());
                        error.set(None);
                        mode.set(TableMode::View);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    html! {
        <>
            <section class="works-table-card">
                <div class="works-table-toolbar">
                    <div class="works-table-toolbar-left">
                        <h3 class="works-table-title">{ "Works" }</h3>
                        if !sorted_works.is_empty() {
                            <span class="works-table-count">
                                { format!("{} record{}", sorted_works.len(),
                                    if sorted_works.len() == 1 { "" } else { "s" }) }
                            </span>
                        }

                        if is_edit {
                            <span class="works-mode-badge works-mode-badge--edit">
                                { "Edit Mode" }
                            </span>
                            if dirty {
                                <span class="works-mode-badge">
                                    { "Unsaved Changes" }
                                </span>
                            }
                        }
                    </div>
                    <div class="works-table-toolbar-right">
                        if is_edit {
                            if some_checked {
                                <span class="works-table-selected-label">
                                    { format!("{} selected", selected_count) }
                                </span>
                            }
                            <button
                                class="btn btn-ghost btn-sm danger"
                                disabled={ !some_checked }
                                onclick={ on_delete }
                                title="Delete selected"
                                aria-label="Delete selected"
                            >
                                <TrashCan />
                                { " Delete"}
                            </button>
                            <button
                                class="btn btn-ghost btn-sm"
                                onclick={ on_save_edits }
                                disabled={ *saving || !dirty }
                            >
                                <Save />
                                { if *saving { " Saving..." } else { " Save" } }
                            </button>
                        }

                        <button class="btn btn-ghost btn-sm" onclick={ on_toggle_mode }>
                            if is_edit {
                                <Eye />
                                { " View" }
                            } else {
                                <EditBox />
                                { " Edit" }
                            }
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
                                { if is_edit { html! { <col style="width: 44px" /> } } else { html! {} } }
                                <col style="width: 120px" />
                                <col style="width: 300px" />
                            </colgroup>
                            <thead>
                                <tr>
                                    { if is_edit { html! {
                                        <th class="works-table-th works-table-th--check">
                                            <input
                                                type="checkbox"
                                                checked={ all_checked }
                                                onchange={ on_select_all }
                                            />
                                        </th>
                                    } } else { html! {} } }
                                    <th class="works-table-th works-table-th--sortable"
                                        onclick={ on_sort_category }>
                                        <span class="works-table-th-inner">
                                            { "Category" }
                                            { sort_icon(
                                                (*sort).column == SortColumn::Category,
                                                &(*sort).dir
                                            ) }
                                        </span>
                                    </th>
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
                                    let id = w.id;
                                    let is_checked = (*selected).contains(&w.id);
                                    let on_row_toggle = on_row_toggle.clone();
                                    let on_field_change = on_field_change.clone();

                                    let on_change_category = {
                                        let on_field_change = on_field_change.clone();
                                        Callback::from(move |e: InputEvent| {
                                            use wasm_bindgen::JsCast;
                                            use web_sys::HtmlSelectElement;
                                            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlSelectElement>() {
                                                on_field_change.emit((id, "category_tag", input.value()));
                                            }
                                        })
                                    };

                                    let on_change_description = {
                                        let on_field_change = on_field_change.clone();
                                        Callback::from(move |e: InputEvent| {
                                            use wasm_bindgen::JsCast;
                                            use web_sys::HtmlInputElement;
                                            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                                on_field_change.emit((id, "description", input.value()));
                                            }
                                        })
                                    };

                                    let row_class = if is_checked {
                                        "works-table-row works-table-row--selected"
                                    } else {
                                        "works-table-row"
                                    };

                                    html! {
                                        <tr key={ w.id.to_string() } class={ row_class }>
                                            { if is_edit { html! {
                                                <td class="works-table-td works-table-td--check">
                                                    <input
                                                        type="checkbox"
                                                        checked={ is_checked }
                                                        onchange={ Callback::from(move |_: Event| {
                                                            on_row_toggle.emit(id);
                                                        }) }
                                                    />
                                                </td>
                                            } } else { html! {} } }
                                            <td class="works-table-td family-td-clip">
                                                { if is_edit { html! {
                                                    <select
                                                        class="cell-input"
                                                        oninput={ on_change_category }
                                                    >
                                                        { for categories.iter().map(|c| {
                                                            let sel = w.category_tag == c.tag;
                                                            html! {
                                                                <option value={ c.tag.clone() } selected={ sel }>
                                                                    { format!("[{}] {}", c.tag, c.name) }
                                                                </option>
                                                            }
                                                        }) }
                                                    </select>
                                                } } else { html! {
                                                    { format!("[{}] {}", &w.category_tag, &w.category_name) }
                                                } } }
                                            </td>
                                            <td class="works-table-td family-td-clip">
                                                { if is_edit { html! {
                                                    <input
                                                        type="text"
                                                        class="cell-input"
                                                        value={ w.description.clone() }
                                                        oninput={ on_change_description }
                                                    />
                                                } } else { html! {
                                                    { &w.description }
                                                } } }
                                            </td>
                                        </tr>
                                    }
                                }) }
                            </tbody>
                        </table>
                    }
                </div>
            </section>
        </>
    }
}
