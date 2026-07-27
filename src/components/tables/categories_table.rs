use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::icons::{EditBox, Eye, Save, TrashCan};
use crate::components::tables::TableMode;
use crate::utils::{fetch_records, invoke};
use models::Category;

async fn delete_categories(ids: Vec<i64>) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args {
        ids: Vec<i64>,
    }
    let args = to_value(&Args { ids }).map_err(|e| e.to_string())?;
    let _ = invoke("delete_categories", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

async fn update_category(id: i64, tag: String, name: String) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        id: i64,
        tag: String,
        name: String,
    }
    let args = to_value(&Args { id, tag, name }).map_err(|e| e.to_string())?;
    let _ = invoke("update_category", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
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
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);

    {
        let categories = categories.clone();
        let saved_categories = saved_categories.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Category>("get_categories").await {
                    Ok(data) => {
                        saved_categories.set(data.clone());
                        categories.set(data.clone());
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

    let dirty = *categories != *saved_categories;
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let misc_id = (*categories)
        .iter()
        .find(|c| c.tag == "MISC")
        .map(|c| c.id)
        .unwrap_or(-1);

    let deletable_count = (*categories).iter().filter(|c| c.id != misc_id).count();
    let all_checked = deletable_count > 0
        && (*categories)
            .iter()
            .filter(|c| c.id != misc_id)
            .all(|c| (*selected).contains(&c.id));

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
        let categories = categories.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            let deletable: Vec<i64> = (*categories)
                .iter()
                .filter(|c| c.id != misc_id)
                .map(|c| c.id.clone())
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
        let saved_categories = saved_categories.clone();
        let error = error.clone();
        let mode = mode.clone();

        let on_category_delete = props.on_category_delete.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let count = to_delete.len();
            let selected = selected.clone();
            let categories = categories.clone();
            let saved_categories = saved_categories.clone();
            let error = error.clone();
            let mode = mode.clone();

            let on_category_delete = on_category_delete.clone();
            spawn_local(async move {
                #[derive(Serialize)]
                struct DialogArgs {
                    message: String,
                    title: String,
                }

                let args = to_value(&DialogArgs {
                    message: format!(
                        "Delete {count} selected categor{}? This cannot be undone.",
                        if count == 1 { "y" } else { "ies" }
                    ),
                    title: "Confirm Delete".to_string(),
                })
                .unwrap();

                let Ok(confirmed) = invoke("show_confirm_dialog", args).await else {
                    return;
                };

                if !from_value::<bool>(confirmed).unwrap_or(false) {
                    return;
                }

                match delete_categories(to_delete).await {
                    Ok(_) => {
                        let remaining: Vec<Category> = (*categories)
                            .iter()
                            .filter(|c| !(*selected).contains(&c.id))
                            .cloned()
                            .collect();
                        categories.set(remaining.clone());
                        saved_categories.set(remaining);
                        selected.set(HashSet::new());
                        error.set(None);
                        mode.set(TableMode::View);
                        on_category_delete.emit(());
                    }
                    Err(e) => error.set(Some(e)),
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
        let on_category_delete = props.on_category_delete.clone();

        Callback::from(move |_: MouseEvent| {
            let snapshot = (*saved_categories).clone();
            let categories = categories.clone();

            let changed: Vec<Category> = (*categories)
                .iter()
                .filter(|c| {
                    !snapshot
                        .iter()
                        .any(|s| s.id == c.id && s.tag == c.tag && s.name == c.name)
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
            let on_category_delete = on_category_delete.clone();
            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for cat in changed {
                    if let Err(e) = update_category(cat.id, cat.tag.clone(), cat.name.clone()).await
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
                    on_category_delete.emit(());
                }
                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let categories = categories.clone();

        Callback::from(move |(id, field, value): (i64, &'static str, String)| {
            categories.set({
                let mut next = (*categories).clone();
                if let Some(c) = next.iter_mut().find(|c| c.id == id) {
                    match field {
                        "tag" => {
                            c.tag = value
                                .chars()
                                .filter(|ch| ch.is_ascii_alphabetic())
                                .collect::<String>()
                                .to_uppercase();
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
                        { " ×" }
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
                            <col style="width: 150px" />  // name
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
                            { for categories.iter().enumerate().map(|(idx, c)| {
                                let id = c.id.clone();
                                let is_misc = c.id == misc_id;
                                let is_checked = (*selected).contains(&c.id);
                                let on_row_toggle = on_row_toggle.clone();
                                let on_field_change = on_field_change.clone();

                                let on_change_tag = {
                                    let on_field_change = on_field_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        use wasm_bindgen::JsCast;
                                        use web_sys::HtmlInputElement;
                                        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                            on_field_change.emit((id, "tag", input.value()));
                                        }
                                    })
                                };

                                let on_change_name = {
                                    let on_field_change = on_field_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        use wasm_bindgen::JsCast;
                                        use web_sys::HtmlInputElement;
                                        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                            on_field_change.emit((id, "name", input.value()));
                                        }
                                    })
                                };

                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ idx.to_string() } class={ row_class }>
                                        { if is_edit { html! {
                                            <td class="works-table-td works-table-td--check">
                                                <input
                                                    type="checkbox"
                                                    checked={ is_checked }
                                                    disabled={ is_misc }
                                                    onchange={
                                                        Callback::from(move |_: Event| {
                                                            on_row_toggle.emit(id);
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
