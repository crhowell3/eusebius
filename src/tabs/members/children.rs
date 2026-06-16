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
    let selected = use_state(HashSet::<String>::new);
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

    let has_family = props.selected_family_id.is_some();

    html! {
        <section class="works-table-card">
            <div class="works-table-toolbar">
                <div class="works-table-toolbar-left">
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
                                html! {
                                    <tr key={ c.id } class="works-table-row">
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
