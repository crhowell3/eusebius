use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::invoke;
use shared::Child;

#[derive(Properties, PartialEq)]
pub struct ChildrenTableProps {
    pub selected_family_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetChildrenArgs {
    family_id: String,
}

async fn fetch_children(family_id: &str) -> Result<Vec<Child>, String> {
    let args = to_value(&GetChildrenArgs {
        family_id: family_id.to_string(),
    })
    .map_err(|e| e.to_string())?;
    let result = invoke("get_children_by_family", args).await;
    from_value::<Vec<Child>>(result).map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveChildrenArgs {
    family_id: String,
    children: Vec<Child>,
}

async fn save_children(family_id: &str, children: Vec<Child>) -> Result<Vec<Child>, String> {
    let args = to_value(&SaveChildrenArgs {
        family_id: family_id.to_string(),
        children,
    })
    .map_err(|e| e.to_string())?;
    let result = invoke("save_children", args).await;
    from_value::<Vec<Child>>(result).map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteChildrenArgs {
    child_ids: Vec<i64>,
}

async fn delete_children(child_ids: Vec<i64>) -> Result<(), String> {
    let args = to_value(&DeleteChildrenArgs { child_ids }).map_err(|e| e.to_string())?;
    let result = invoke("delete_children", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

enum FieldValue {
    Text(String),
    Bool(bool),
}

#[function_component(Children)]
pub fn children(props: &ChildrenTableProps) -> Html {
    let children = use_state(Vec::<Child>::new);
    let error = use_state(|| None::<String>);
    let saving = use_state(|| false);
    let dirty = use_state(|| false);
    let next_temp_id = use_state(|| -1i64);
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);

    {
        let children = children.clone();
        let error = error.clone();
        let dirty = dirty.clone();
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
                                children.set(data);
                                error.set(None);
                                dirty.set(false);
                                selected.set(HashSet::new());
                            }
                            Err(e) => error.set(Some(e)),
                        }
                        initial_load.set(false);
                    });
                }
                None => {
                    children.set(Vec::new());
                    error.set(None);
                    dirty.set(false);
                    initial_load.set(false);
                }
            }

            || ()
        });
    }

    let on_field_change = {
        let children = children.clone();
        let dirty = dirty.clone();
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
            dirty.set(true);
        })
    };

    let on_add_row = {
        let children = children.clone();
        let dirty = dirty.clone();
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
            dirty.set(true);
        })
    };

    let on_save = {
        let children = children.clone();
        let error = error.clone();
        let dirty = dirty.clone();
        let saving = saving.clone();
        let family_id = props.selected_family_id.clone();
        Callback::from(move |_: MouseEvent| {
            let Some(fid) = family_id.clone() else { return };
            let payload = (*children).clone();
            let children = children.clone();
            let error = error.clone();
            let dirty = dirty.clone();
            let saving = saving.clone();
            saving.set(true);
            spawn_local(async move {
                match save_children(&fid, payload).await {
                    Ok(refreshed) => {
                        children.set(refreshed);
                        error.set(None);
                        dirty.set(false);
                    }
                    Err(e) => error.set(Some(e)),
                }
                saving.set(false);
            });
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

    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let all_checked =
        !(*children).is_empty() && (*children).iter().all(|c| (*selected).contains(&c.id));

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
        let dirty = dirty.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let children = children.clone();
            let dirty = dirty.clone();
            spawn_local(async move {
                match delete_children(to_delete).await {
                    Ok(_) => {
                        let remaining = (*children)
                            .iter()
                            .filter(|c| !(*selected).contains(&c.id))
                            .cloned()
                            .collect();
                        children.set(remaining);
                        selected.set(HashSet::new());
                        dirty.set(false);
                    }
                    Err(_e) => {
                        // TODO: proper error handling
                    }
                }
            });
        })
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
                    if *dirty {
                        <span class="works-table-selected-label">{ "Unsaved changes" }</span>
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
                    <button
                        class="btn btn-primary btn-sm"
                        disabled={ !*dirty || *saving || !has_family }
                        onclick={ on_save }
                    >
                        { if *saving { "Saving..." } else { "Save Changes" } }
                    </button>
                </div>
            </div>

            if let Some(err) = (*error).clone() {
                <div class="works-table-error">{ err }</div>
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
                } else {
                    <table class="family-table">
                        <colgroup>
                            <col style="width: 44px" />   // checkbox
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
                                <th class="works-table-th works-table-th--check">
                                    <input type="checkbox" checked={ all_checked } onchange={ on_select_all } />
                                </th>
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
                        </tbody>
                    </table>
                }
            </div>
        </section>
    }
}
