use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::icons::{EditBox, Eye, Save, TrashCan};
use crate::utils::{fetch_records, invoke};
use models::Category;

async fn delete_categories(tags: Vec<String>) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        tags: Vec<String>,
    }
    let args = to_value(&Args { tags }).map_err(|e| e.to_string())?;
    let _ = invoke("delete_categories", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

async fn update_category(old_tag: String, new_tag: String, name: String) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        old_tag: String,
        new_tag: String,
        name: String,
    }
    let args = to_value(&Args {
        old_tag,
        new_tag,
        name,
    })
    .map_err(|e| e.to_string())?;
    let _ = invoke("update_category", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

#[derive(Clone, PartialEq)]
enum TableMode {
    View,
    Edit,
}

#[derive(Properties, PartialEq)]
pub struct CategoriesTableProps {
    pub refresh_trigger: u32,
    pub on_category_delete: Callback<()>,
}

#[function_component(CategoriesTable)]
pub fn categories_table(props: &CategoriesTableProps) -> Html {
    let categories = use_state(Vec::<Category>::new);
    let saved_categories = use_state(Vec::<Category>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);
    let original_tags = use_state(std::collections::HashMap::<String, String>::new);

    {
        let categories = categories.clone();
        let saved_categories = saved_categories.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;
        let original_tags = original_tags.clone();

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Category>("get_categories").await {
                    Ok(data) => {
                        saved_categories.set(data.clone());
                        categories.set(data.clone());
                        error.set(None);
                        selected.set(HashSet::new());
                        original_tags.set(
                            data.iter()
                                .map(|c| (c.tag.clone(), c.tag.clone()))
                                .collect(),
                        );
                    }
                    Err(e) => error.set(Some(e)),
                }
                initial_load.set(false);
            });
            || ()
        });
    }

    let dirty = *categories != *saved_categories;
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let can_delete = some_checked && !(*selected).contains("MISC");
    let all_checked = can_delete
        && (*categories)
            .iter()
            .filter(|c| c.tag != "MISC")
            .all(|c| (*selected).contains(&c.tag));

    let on_toggle_mode = {
        let mode = mode.clone();
        let categories = categories.clone();
        let saved_categories = saved_categories.clone();
        let selected = selected.clone();

        Callback::from(move |_: MouseEvent| {
            if *mode == TableMode::Edit {
                categories.set((*saved_categories).clone());
                selected.set(HashSet::new());
                mode.set(TableMode::View);
            } else {
                mode.set(TableMode::Edit);
            }
        })
    };

    let on_row_toggle = {
        let selected = selected.clone();
        Callback::from(move |tag: String| {
            if tag == "MISC" {
                return;
            }
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
        let categories = categories.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            let deletable: Vec<String> = (*categories)
                .iter()
                .filter(|c| c.tag != "MISC")
                .map(|c| c.tag.clone())
                .collect();
            if deletable.iter().all(|t| (*selected).contains(t)) {
                selected.set(HashSet::new());
            } else {
                selected.set(deletable.into_iter().collect());
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let categories = categories.clone();
        let on_category_delete = props.on_category_delete.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let works = categories.clone();
            let on_category_delete = on_category_delete.clone();
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
                        on_category_delete.emit(());
                    }
                    Err(_e) => {
                        // TODO(@crhowell3): Add in proper error handling
                    }
                }
            });
        })
    };

    let on_save_edits = {
        let categories = categories.clone();
        let saved_categories = saved_categories.clone();
        let error = error.clone();
        let saving = saving.clone();
        let mode = mode.clone();
        let saved_snapshot = (*saved_categories).clone();
        let original_tags = original_tags.clone();

        Callback::from(move |_: MouseEvent| {
            let changed: Vec<Category> = (*categories)
                .iter()
                .filter(|w| {
                    !saved_snapshot
                        .iter()
                        .any(|s| s.tag == w.tag && s.name == w.name)
                })
                .cloned()
                .collect();

            if changed.is_empty() {
                mode.set(TableMode::View);
                return;
            }

            let categories = categories.clone();
            let saved_categories = saved_categories.clone();
            let error = error.clone();
            let saving = saving.clone();
            let mode = mode.clone();
            let original_tags = original_tags.clone();
            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for cat in changed {
                    let old_tag = (*original_tags)
                        .get(&cat.tag)
                        .cloned()
                        .unwrap_or_else(|| cat.tag.clone());
                    if let Err(e) =
                        update_category(old_tag, cat.tag.clone(), cat.name.clone()).await
                    {
                        error.set(Some(e));
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    saved_categories.set((*categories).clone());
                    error.set(None);
                    mode.set(TableMode::View);
                }
                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let categories = categories.clone();

        Callback::from(move |(tag, field, value): (String, &'static str, String)| {
            categories.set({
                let mut next = (*categories).clone();
                if let Some(c) = next.iter_mut().find(|c| c.tag == tag) {
                    match field {
                        "tag" => {
                            let val = value
                                .chars()
                                .filter(|c| c.is_ascii_alphabetic())
                                .collect::<String>()
                                .to_uppercase();
                            c.tag = val;
                        }
                        "name" => c.name = value,
                        _ => {}
                    }
                }
                next
            });
        })
    };

    html! {
        <section class="works-table-card">
            <div class="works-table-toolbar">
                <div class="works-table-toolbar-left">
                    <h3 class="works-table-title">{ "Categories" }</h3>
                    if !categories.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", categories.len(),
                                if categories.len() == 1 { "" } else { "s" }) }
                        </span>
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

            if let Some(err) = (*error).as_deref() {
                <div class="works-table-error">
                    <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="8" x2="12" y2="12"/>
                        <line x1="12" y1="16" x2="12.01" y2="16"/>
                    </svg>
                    { err }
                    <button
                        style="margin-left: auto; background: none; border: none; cursor: pointer; color: inherit;"
                        onclick={ Callback::from({
                            let error = error.clone();
                            move |_: MouseEvent| error.set(None)
                        }) }
                    >
                        { "×" }
                    </button>
                </div>
            }

            <div class="works-table-body">
                if *initial_load {
                    <div class="works-table-empty">
                        <span class="works-table-empty-text">{ "Loading..." }</span>
                    </div>
                } else if categories.is_empty() {
                    <div class="works-table-empty">
                        <span class="works-table-empty-text">{ "No categories" }</span>
                        <span class="works-table-empty-sub">{ "If you see this, it's a bug. Report to me@crhowell.com" }</span>
                    </div>
                } else {
                    <table class="works-table">
                        <colgroup>
                            { if is_edit { html! { <col style="width: 44px" /> } } else { html! {} } }
                            <col style="width: 80px" />   // tag
                            <col class="width: 150px" />  // name
                            <col style="width: 100px" />  // protected badge
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
                                <th class="works-table-th">{ "Tag" }</th>
                                <th class="works-table-th">{ "Name" }</th>
                                <th class="works-table-th"></th>
                            </tr>
                        </thead>
                        <tbody>
                            { for categories.iter().map(|c| {
                                let tag = c.tag.clone();
                                let is_misc = c.tag == "MISC";
                                let is_checked = (*selected).contains(&c.tag);
                                let on_row_toggle = on_row_toggle.clone();
                                let on_field_change = on_field_change.clone();

                                let on_change_tag = {
                                    let on_field_change = on_field_change.clone();
                                    let tag = tag.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let tag = tag.clone();
                                        use wasm_bindgen::JsCast;
                                        use web_sys::HtmlInputElement;
                                        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                            on_field_change.emit((tag, "tag", input.value()));
                                        }
                                    })
                                };

                                let on_change_name = {
                                    let on_field_change = on_field_change.clone();
                                    let tag = tag.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let tag = tag.clone();
                                        use wasm_bindgen::JsCast;
                                        use web_sys::HtmlInputElement;
                                        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                            on_field_change.emit((tag, "name", input.value()));
                                        }
                                    })
                                };

                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ c.tag.clone() } class={ row_class }>
                                        { if is_edit { html! {
                                            <td class="works-table-td works-table-td--check">
                                                <input
                                                    type="checkbox"
                                                    checked={ is_checked }
                                                    disabled={ is_misc }
                                                    onchange={
                                                        Callback::from(move |_: Event| {
                                                            on_row_toggle.emit(tag.clone());
                                                        })
                                                    }
                                                />
                                            </td>
                                        } } else { html! {} } }
                                        <td class="works-table-td">
                                            { if is_edit && !is_misc { html! {
                                                <input
                                                    type="text"
                                                    class="cell-input"
                                                    value={ c.tag.clone() }
                                                    oninput={ on_change_tag }
                                                />
                                            } } else { html! {
                                                <span class="works-table-code-badge">
                                                    { &c.tag }
                                                </span>
                                            } } }
                                        </td>
                                        <td class="works-table-td">
                                            { if is_edit && !is_misc { html! {
                                                <input
                                                    type="text"
                                                    class="cell-input"
                                                    value={ c.name.clone() }
                                                    oninput={ on_change_name }
                                                />
                                            } } else { html! {
                                                { &c.name }
                                            } } }
                                        </td>
                                        <td class="works-table-td">
                                            if is_misc {
                                                <span class="family-badge family-badge--no">
                                                    { "Protected" }
                                                </span>
                                            }
                                        </td>
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
