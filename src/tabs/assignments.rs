use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;
use models::{Family, MemberWorkView, Person, Work};

async fn fetch_families() -> Result<Vec<Family>, String> {
    let result = invoke("get_families", JsValue::UNDEFINED)
        .await
        .map_err(|e| e.as_string().unwrap_or_default())?;
    from_value::<Vec<Family>>(result).map_err(|e| e.to_string())
}

async fn fetch_works() -> Result<Vec<Work>, String> {
    let result = invoke("get_works", JsValue::UNDEFINED)
        .await
        .map_err(|e| e.as_string().unwrap_or_default())?;
    from_value::<Vec<Work>>(result).map_err(|e| e.to_string())
}

async fn fetch_persons_for_family(family_id: &str) -> Result<Vec<Person>, String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { family_id: String }
    let args = to_value(&Args { family_id: family_id.to_string() }).map_err(|e| e.to_string())?;
    let result = invoke("get_persons_for_family", args)
        .await
        .map_err(|e| e.as_string().unwrap_or_default())?;
    from_value::<Vec<Person>>(result).map_err(|e| e.to_string())
}

async fn fetch_works_for_person(person_id: i64) -> Result<Vec<MemberWorkView>, String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { person_id: i64 }
    let args = to_value(&Args { person_id }).map_err(|e| e.to_string())?;
    let result = invoke("get_works_for_person", args)
        .await
        .map_err(|e| e.as_string().unwrap_or_default())?;
    from_value::<Vec<MemberWorkView>>(result).map_err(|e| e.to_string())
}

async fn set_works_for_person(person_id: i64, work_ids: Vec<i64>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { person_id: i64, work_ids: Vec<i64> }
    let args = to_value(&Args { person_id, work_ids }).map_err(|e| e.to_string())?;
    let _ = invoke("set_works_for_person", args)
        .await
        .map_err(|e| e.as_string().unwrap_or_default())?;
    Ok(())
}

#[function_component(AssignmentsTab)]
pub fn assignments_tab() -> Html {
    let families         = use_state(Vec::<Family>::new);
    let all_works        = use_state(Vec::<Work>::new);
    let persons          = use_state(Vec::<Person>::new);
    let selected_family  = use_state(|| None::<String>);
    let selected_person  = use_state(|| None::<i64>);
    let assigned_ids     = use_state(HashSet::<i64>::new); // current saved assignments
    let pending_ids      = use_state(HashSet::<i64>::new); // checkbox state (unsaved)
    let loading_persons  = use_state(|| false);
    let loading_works    = use_state(|| false);
    let saving           = use_state(|| false);
    let error            = use_state(|| None::<String>);

    let dirty = *pending_ids != *assigned_ids;

    {
        let families  = families.clone();
        let all_works = all_works.clone();
        let error     = error.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_families().await {
                    Ok(data) => families.set(data),
                    Err(e)   => error.set(Some(e)),
                }
                match fetch_works().await {
                    Ok(data) => all_works.set(data),
                    Err(e)   => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    {
        let persons         = persons.clone();
        let selected_person = selected_person.clone();
        let assigned_ids    = assigned_ids.clone();
        let pending_ids     = pending_ids.clone();
        let loading_persons = loading_persons.clone();
        let error           = error.clone();
        let family_id       = (*selected_family).clone();

        use_effect_with(family_id, move |family_id| {
            if let Some(fid) = family_id.clone() {
                loading_persons.set(true);
                selected_person.set(None);
                assigned_ids.set(HashSet::new());
                pending_ids.set(HashSet::new());
                spawn_local(async move {
                    match fetch_persons_for_family(&fid).await {
                        Ok(data) => { persons.set(data); error.set(None); }
                        Err(e)   => error.set(Some(e)),
                    }
                    loading_persons.set(false);
                });
            } else {
                persons.set(Vec::new());
                selected_person.set(None);
                assigned_ids.set(HashSet::new());
                pending_ids.set(HashSet::new());
            }
            || ()
        });
    }

    {
        let assigned_ids   = assigned_ids.clone();
        let pending_ids    = pending_ids.clone();
        let loading_works  = loading_works.clone();
        let error          = error.clone();
        let person_id      = (*selected_person).clone();

        use_effect_with(person_id, move |person_id| {
            if let Some(pid) = *person_id {
                loading_works.set(true);
                spawn_local(async move {
                    match fetch_works_for_person(pid).await {
                        Ok(data) => {
                            let ids: HashSet<i64> = data.iter().map(|w| w.work_id).collect();
                            assigned_ids.set(ids.clone());
                            pending_ids.set(ids);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(e)),
                    }
                    loading_works.set(false);
                });
            } else {
                assigned_ids.set(HashSet::new());
                pending_ids.set(HashSet::new());
            }
            || ()
        });
    }

    let on_select_family = {
        let selected_family = selected_family.clone();
        Callback::from(move |family_id: String| {
            selected_family.set(Some(family_id));
        })
    };

    let on_select_person = {
        let selected_person = selected_person.clone();
        Callback::from(move |person_id: i64| {
            selected_person.set(Some(person_id));
        })
    };

    let on_toggle_work = {
        let pending_ids = pending_ids.clone();
        Callback::from(move |work_id: i64| {
            let mut next = (*pending_ids).clone();
            if next.contains(&work_id) { next.remove(&work_id); } else { next.insert(work_id); }
            pending_ids.set(next);
        })
    };

    let on_save = {
        let pending_ids     = pending_ids.clone();
        let assigned_ids    = assigned_ids.clone();
        let selected_person = selected_person.clone();
        let saving          = saving.clone();
        let error           = error.clone();
        Callback::from(move |_: MouseEvent| {
            let Some(pid) = *selected_person else { return };
            let work_ids = (*pending_ids).iter().cloned().collect::<Vec<_>>();
            let pending_ids  = pending_ids.clone();
            let assigned_ids = assigned_ids.clone();
            let saving       = saving.clone();
            let error        = error.clone();
            saving.set(true);
            spawn_local(async move {
                match set_works_for_person(pid, work_ids).await {
                    Ok(_) => {
                        assigned_ids.set((*pending_ids).clone());
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
                saving.set(false);
            });
        })
    };

    let on_discard = {
        let pending_ids  = pending_ids.clone();
        let assigned_ids = assigned_ids.clone();
        Callback::from(move |_: MouseEvent| {
            pending_ids.set((*assigned_ids).clone());
        })
    };

    let mut grouped: Vec<(String, Vec<&Work>)> = Vec::new();
    for work in (*all_works).iter() {
        if let Some((_, works)) = grouped.iter_mut()
            .find(|(name, _)| name == &work.category_name)
        {
            works.push(work);
        } else {
            grouped.push((work.category_name.clone(), vec![work]));
        }
    }

    html! {
        <div class="assignments-root">

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
                        onclick={ Callback::from({ let e = error.clone(); move |_: MouseEvent| e.set(None) }) }
                    >{ "×" }</button>
                </div>
            }

            <div class="assignments-layout">

                <div class="assignments-col">
                    <div class="assignments-col-header">
                        <h3 class="assignments-col-title">{ "Family" }</h3>
                        <span class="works-table-count">
                            { format!("{} records", (*families).len()) }
                        </span>
                    </div>
                    <div class="assignments-list">
                        { for (*families).iter().map(|f| {
                            let fid = f.family_id.clone();
                            let is_selected = (*selected_family).as_deref() == Some(&f.family_id);
                            let on_select = on_select_family.clone();
                            html! {
                                <button
                                    key={ f.family_id.clone() }
                                    class={ if is_selected {
                                        "assignments-list-item assignments-list-item--selected"
                                    } else {
                                        "assignments-list-item"
                                    }}
                                    onclick={ Callback::from(move |_: MouseEvent| {
                                        on_select.emit(fid.clone());
                                    }) }
                                >
                                    <span class="works-table-code-badge">
                                        { &f.family_id }
                                    </span>
                                    <span class="assignments-list-name">
                                        { format!("{}, {}", f.last_name, f.first_name) }
                                    </span>
                                </button>
                            }
                        }) }
                    </div>
                </div>

                <div class="assignments-col">
                    <div class="assignments-col-header">
                        <h3 class="assignments-col-title">{ "Person" }</h3>
                    </div>
                    <div class="assignments-list">
                        if (*selected_family).is_none() {
                            <p class="assignments-empty">
                                { "Select a family to see its members" }
                            </p>
                        } else if *loading_persons {
                            <p class="assignments-empty">{ "Loading..." }</p>
                        } else if (*persons).is_empty() {
                            <p class="assignments-empty">{ "No persons found" }</p>
                        } else {
                            { for (*persons).iter().map(|p| {
                                let pid = p.id;
                                let is_selected = (*selected_person) == Some(p.id);
                                let on_select = on_select_person.clone();
                                let role_badge = match p.role.as_str() {
                                    "head"   => "Head",
                                    "spouse" => "Spouse",
                                    "child"  => "Child",
                                    _        => "Member",
                                };
                                html! {
                                    <button
                                        key={ p.id.to_string() }
                                        class={ if is_selected {
                                            "assignments-list-item assignments-list-item--selected"
                                        } else {
                                            "assignments-list-item"
                                        }}
                                        onclick={ Callback::from(move |_: MouseEvent| {
                                            on_select.emit(pid);
                                        }) }
                                    >
                                        <span class="assignments-role-badge">
                                            { role_badge }
                                        </span>
                                        <span class="assignments-list-name">
                                            { format!("{} {}", p.first_name, p.last_name) }
                                        </span>
                                    </button>
                                }
                            }) }
                        }
                    </div>
                </div>

                <div class="assignments-col assignments-col--wide">
                    <div class="assignments-col-header">
                        <h3 class="assignments-col-title">{ "Works" }</h3>
                        if dirty {
                            <span class="works-table-selected-label">
                                { "Unsaved changes" }
                            </span>
                        }
                        <div style="margin-left: auto; display: flex; gap: var(--space-2);">
                            if dirty {
                                <button
                                    class="btn btn-ghost btn-sm"
                                    onclick={ on_discard }
                                >
                                    { "Discard" }
                                </button>
                            }
                            <button
                                class="btn btn-primary btn-sm"
                                onclick={ on_save }
                                disabled={ !dirty || *saving || (*selected_person).is_none() }
                            >
                                { if *saving { "Saving..." } else { "Save Assignments" } }
                            </button>
                        </div>
                    </div>

                    <div class="assignments-works-body">
                        if (*selected_person).is_none() {
                            <div class="works-table-empty">
                                <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28"
                                    viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                    stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
                                    class="works-table-empty-icon">
                                    <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                                    <circle cx="9" cy="7" r="4"/>
                                    <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                                    <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
                                </svg>
                                <span class="works-table-empty-text">
                                    { "Select a person to manage their works" }
                                </span>
                            </div>
                        } else if *loading_works {
                            <div class="works-table-empty">
                                <span class="works-table-empty-text">{ "Loading..." }</span>
                            </div>
                        } else if (*all_works).is_empty() {
                            <div class="works-table-empty">
                                <span class="works-table-empty-text">{ "No works defined yet" }</span>
                                <span class="works-table-empty-sub">
                                    { "Add works in the Works tab first" }
                                </span>
                            </div>
                        } else {
                            { for grouped.iter().map(|(category_name, works)| {
                                let on_toggle = on_toggle_work.clone();
                                let pending = (*pending_ids).clone();
                                html! {
                                    <div class="assignments-category-group" key={ category_name.clone() }>
                                        <div class="assignments-category-label">
                                            { category_name }
                                        </div>
                                        { for works.iter().map(|w| {
                                            let wid = w.id;
                                            let is_checked = pending.contains(&w.id);
                                            let on_toggle = on_toggle.clone();
                                            html! {
                                                <label
                                                    key={ w.id.to_string() }
                                                    class={ if is_checked {
                                                        "assignments-work-item assignments-work-item--checked"
                                                    } else {
                                                        "assignments-work-item"
                                                    }}
                                                >
                                                    <input
                                                        type="checkbox"
                                                        checked={ is_checked }
                                                        onchange={ Callback::from(move |_: Event| {
                                                            on_toggle.emit(wid);
                                                        }) }
                                                    />
                                                    <span class="works-table-code-badge">
                                                        { &w.category_tag }
                                                    </span>
                                                    <span class="assignments-work-desc">
                                                        { &w.description }
                                                    </span>
                                                </label>
                                            }
                                        }) }
                                    </div>
                                }
                            }) }
                        }
                    </div>
                </div>

            </div>
        </div>
    }
}
