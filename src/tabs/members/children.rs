use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::icons::{AddRecord, EditBox, ErrorIcon, Eye, Save, TrashCan};
use crate::components::tables::TableMode;
use crate::utils::{format_phone, invoke};
use models::Child;

async fn fetch_children(family_id: &str) -> Result<Vec<Child>, String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        family_id: String,
    }
    let args = to_value(&Args {
        family_id: family_id.to_string(),
    })
    .map_err(|e| e.to_string())?;
    let result = invoke("get_children_by_family", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;
    from_value::<Vec<Child>>(result).map_err(|e| e.to_string())
}

async fn update_child(child: Child) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        child: Child,
    }
    let args = to_value(&Args { child }).map_err(|e| e.to_string())?;
    let _ = invoke("update_child", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

async fn delete_children(child_ids: Vec<i64>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        child_ids: Vec<i64>,
    }
    let args = to_value(&Args { child_ids }).map_err(|e| e.to_string())?;
    let _ = invoke("delete_children", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

enum FieldValue {
    Text(String),
    Bool(bool),
}

#[derive(Properties, PartialEq)]
pub struct ChildrenTableProps {
    pub selected_family_id: Option<String>,
}

#[function_component(Children)]
pub fn children(props: &ChildrenTableProps) -> Html {
    let children = use_state(Vec::<Child>::new);
    let saved_children = use_state(Vec::<Child>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);
    let next_temp_id = use_state(|| -1i64);

    {
        let children = children.clone();
        let saved_children = saved_children.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let family_id = props.selected_family_id.clone();

        use_effect_with(family_id.clone(), move |family_id| {
            let family_id = family_id.clone();
            match family_id {
                Some(id) => {
                    initial_load.set(true);
                    spawn_local(async move {
                        match fetch_children(&id).await {
                            Ok(data) => {
                                saved_children.set(data.clone());
                                children.set(data);
                                error.set(None);
                                selected.set(HashSet::new());
                            }
                            Err(e) => error.set(Some(e)),
                        }
                        initial_load.set(false);
                    });
                }
                None => {
                    saved_children.set(Vec::new());
                    children.set(Vec::new());
                    error.set(None);
                    initial_load.set(false);
                }
            }

            || ()
        });
    }

    let dirty = *children != *saved_children;
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let all_checked =
        !(*children).is_empty() && (*children).iter().all(|c| (*selected).contains(&c.id));

    let on_toggle_mode = {
        let mode = mode.clone();
        let children = children.clone();
        let saved_children = saved_children.clone();
        let selected = selected.clone();

        Callback::from(move |_: MouseEvent| {
            if *mode == TableMode::Edit {
                children.set((*saved_children).clone());
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
        let children = children.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if (*children).iter().all(|c| (*selected).contains(&c.id)) {
                selected.set(HashSet::new());
            } else {
                selected.set((*children).iter().map(|c| c.id).collect());
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let children = children.clone();
        let saved_children = saved_children.clone();
        let error = error.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let count = to_delete.len();
            let selected = selected.clone();
            let children = children.clone();
            let saved_children = saved_children.clone();
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
                        "Delete {count} selected famil{}? All associated records will also be deleted. This cannot be undone.",
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

                match delete_children(to_delete).await {
                    Ok(_) => {
                        let remaining: Vec<Child> = (*children)
                            .iter()
                            .filter(|c| !(*selected).contains(&c.id))
                            .cloned()
                            .collect();
                        children.set(remaining.clone());
                        saved_children.set(remaining);
                        selected.set(HashSet::new());
                        error.set(None);
                        mode.set(TableMode::View);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    let on_save_edits = {
        let children = children.clone();
        let saved_children = saved_children.clone();
        let error = error.clone();
        let saving = saving.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let snapshot = (*saved_children).clone();
            let children = children.clone();

            let changed: Vec<Child> = (*children)
                .iter()
                .filter(|c| {
                    !snapshot.iter().any(|n| {
                        n.family_id == c.family_id
                            && n.first_name == c.first_name
                            && n.last_name == c.last_name
                            && n.date_of_birth == c.date_of_birth
                            && n.cell_phone == c.cell_phone
                            && n.work_phone == c.work_phone
                            && n.email_address == c.email_address
                            && n.is_member == c.is_member
                            && n.is_active == c.is_active
                            && n.on_bulletin_email_list == c.on_bulletin_email_list
                    })
                })
                .cloned()
                .collect();

            if changed.is_empty() {
                mode.set(TableMode::View);
                return;
            }

            let children = children.clone();
            let saved_children = saved_children.clone();
            let error = error.clone();
            let saving = saving.clone();
            let mode = mode.clone();

            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for child in changed {
                    if let Err(e) = update_child(child).await {
                        error.set(Some(e));
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    saved_children.set((*children).clone());
                    error.set(None);
                    mode.set(TableMode::View);
                }

                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let children = children.clone();
        Callback::from(move |(idx, field, value): (usize, String, FieldValue)| {
            let mut next = (*children).clone();
            if let Some(child) = next.get_mut(idx) {
                match (field.as_str(), value) {
                    ("first_name", FieldValue::Text(v)) => child.first_name = v,
                    ("last_name", FieldValue::Text(v)) => child.last_name = v,
                    ("date_of_birth", FieldValue::Text(v)) => child.date_of_birth = v,
                    ("cell_phone", FieldValue::Text(v)) => child.cell_phone = v,
                    ("work_phone", FieldValue::Text(v)) => child.work_phone = v,
                    ("email_address", FieldValue::Text(v)) => child.email_address = v,
                    ("is_member", FieldValue::Bool(v)) => child.is_member = v,
                    ("is_active", FieldValue::Bool(v)) => child.is_active = v,
                    ("on_bulletin_email_list", FieldValue::Bool(v)) => {
                        child.on_bulletin_email_list = v
                    }
                    _ => {}
                }
            }
            children.set(next);
        })
    };

    let on_add_row = {
        let children = children.clone();
        let next_temp_id = next_temp_id.clone();
        let family_id = props.selected_family_id.clone();
        Callback::from(move |_: MouseEvent| {
            let Some(fid) = family_id.clone() else { return };
            let mut next = (*children).clone();
            next.push(Child {
                id: *next_temp_id,
                family_id: fid,
                ..Default::default()
            });
            next_temp_id.set(*next_temp_id - 1);
            children.set(next);
        })
    };

    let text_input = |idx: usize,
                      field: &'static str,
                      value: &str,
                      on_change: &Callback<(usize, String, FieldValue)>| {
        let on_change = on_change.clone();
        let oninput = Callback::from(move |e: InputEvent| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            on_change.emit((idx, field.to_string(), FieldValue::Text(input.value())));
        });
        html! {
            <input
                type="text"
                class="cell-input"
                placeholder="—"
                value={ value.to_string() }
                oninput={ oninput }
            />
        }
    };

    let bool_input = |idx: usize,
                      field: &'static str,
                      value: bool,
                      on_change: &Callback<(usize, String, FieldValue)>| {
        let on_change = on_change.clone();
        let onchange = Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            on_change.emit((idx, field.to_string(), FieldValue::Bool(input.checked())));
        });
        html! {
            <input
                type="checkbox"
                class="cell-checkbox"
                checked={ value }
                onchange={ onchange }
            />
        }
    };

    let has_family = props.selected_family_id.is_some();

    html! {
        <section class="works-table-card">
            <div class="works-table-toolbar">
                <div class="works-table-toolbar-left">
                    <span class="works-form-header-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                            <circle cx="12" cy="7" r="4"/>
                        </svg>
                    </span>
                    <h3 class="works-table-title">{ "Children" }</h3>
                    if let Some(fid) = &props.selected_family_id {
                        <span class="works-table-count">
                            { format!("Family {}", fid) }
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
                    <ErrorIcon />
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
                if !has_family {
                    <div class="works-table-empty">
                        <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
                            class="works-table-empty-icon">
                            <circle cx="12" cy="8" r="4"/>
                            <path d="M4 21v-2a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v2"/>
                        </svg>
                        <span class="works-table-empty-text">{ "No family selected" }</span>
                        <span class="works-table-empty-sub">
                            { "Double-click a family record to view its children" }
                        </span>
                    </div>
                } else if *initial_load {
                    <div class="works-table-empty">
                        <span class="works-table-empty-text">{ "Loading..." }</span>
                    </div>
                } else if !is_edit {
                    <div class="works-table-empty">
                        <AddRecord />
                        <span class="works-table-empty-text">
                            <tspan>{ "Click the " }</tspan>
                            <EditBox />
                            <tspan>{ " Edit button to begin" }</tspan>
                        </span>
                    </div>
                } else {
                    <table class="family-table">
                        <colgroup>
                            { if is_edit { html! { <col style="width: 44px" /> } } else { html! {} } }
                            <col style="width: 130px" />  // first name
                            <col style="width: 130px" />  // last name
                            <col style="width: 60px" />   // member
                            <col style="width: 60px" />   // active
                            <col style="width: 110px" />  // dob
                            <col style="width: 120px" />  // cell phone
                            <col style="width: 120px" />  // work phone
                            <col style="width: 180px" />  // email
                            <col style="width: 80px" />   // bulletin
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
                                <th class="works-table-th">{ "First Name" }</th>
                                <th class="works-table-th">{ "Last Name" }</th>
                                <th class="works-table-th">{ "Mbr?" }</th>
                                <th class="works-table-th">{ "Active?" }</th>
                                <th class="works-table-th">{ "DOB" }</th>
                                <th class="works-table-th">{ "Cell Ph." }</th>
                                <th class="works-table-th">{ "Work Ph." }</th>
                                <th class="works-table-th">{ "Email" }</th>
                                <th class="works-table-th">{ "Bulletin?" }</th>
                            </tr>
                        </thead>
                        <tbody>
                        { for (*children).iter().enumerate().map(|(idx, c)| {
                                let child_id = c.id;
                                let is_checked = (*selected).contains(&c.id);
                                let on_row_toggle = on_row_toggle.clone();
                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };
                                html! {
                                    <tr key={ c.id } class={ row_class }>
                                        <td class="works-table-td works-table-td--check">
                                            <input
                                                type="checkbox"
                                                checked={ is_checked }
                                                onchange={ Callback::from(move |_: Event| {
                                                    on_row_toggle.emit(child_id);
                                                }) }
                                            />
                                        </td>
                                        <td class="works-table-td">
                                            { text_input(idx, "first_name", &c.first_name, &on_field_change) }
                                        </td>
                                        <td class="works-table-td">
                                            { text_input(idx, "last_name", &c.last_name, &on_field_change) }
                                        </td>
                                        <td class="works-table-td works-table-td--check">
                                            { bool_input(idx, "is_member", c.is_member, &on_field_change) }
                                        </td>
                                        <td class="works-table-td works-table-td--check">
                                            { bool_input(idx, "is_active", c.is_active, &on_field_change) }
                                        </td>
                                        <td class="works-table-td">
                                            { text_input(idx, "date_of_birth", &c.date_of_birth, &on_field_change) }
                                        </td>
                                        <td class="works-table-td">
                                            { text_input(idx, "cell_phone", &c.cell_phone, &on_field_change) }
                                        </td>
                                        <td class="works-table-td">
                                            { text_input(idx, "work_phone", &c.work_phone, &on_field_change) }
                                        </td>
                                        <td class="works-table-td">
                                            { text_input(idx, "email_address", &c.email_address, &on_field_change) }
                                        </td>
                                        <td class="works-table-td works-table-td--check">
                                            { bool_input(idx, "on_bulletin_email_list", c.on_bulletin_email_list, &on_field_change) }
                                        </td>
                                    </tr>
                                }
                            }) }

                            { if is_edit
                                {
                                    html! {
                                        <tr class="works-table-row family-add-row" onclick={ on_add_row }>
                                            <td class="works-table-td family-add-cell" colspan="9">
                                                <span class="family-add-content">
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                                                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                                        stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                                        <line x1="12" y1="5" x2="12" y2="19" />
                                                        <line x1="5" y1="12" x2="19" y2="12" />
                                                    </svg>
                                                    { "Click to add record" }
                                                </span>
                                            </td>
                                        </tr>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </tbody>
                    </table>
                }
            </div>
        </section>
    }
}
