use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::sorting::{SortDir, SortState, sort_icon};
use crate::on_sort;
use crate::utils::{fetch_records, invoke};
use models::Baptism;

#[derive(Properties, PartialEq)]
pub struct BaptismsTableProps {
    pub refresh_trigger: u32,
}

async fn delete_baptisms(family_ids: Vec<String>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        family_ids: Vec<String>,
    }
    let args = to_value(&Args { family_ids }).map_err(|e| e.to_string())?;
    let result = invoke("delete_baptisms", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Default, Clone, PartialEq)]
enum SortColumn {
    #[default]
    FamilyId,
    LastName,
    FirstName,
    DateBaptized,
    Witness,
    Location,
}

fn sort_baptisms(baptisms: &[Baptism], sort: &SortState<SortColumn>) -> Vec<Baptism> {
    let mut sorted = baptisms.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::FamilyId => a.family_id.cmp(&b.family_id),
            SortColumn::LastName => a.last_name.cmp(&b.last_name),
            SortColumn::FirstName => a.first_name.cmp(&b.first_name),
            SortColumn::DateBaptized => a.date_baptized.cmp(&b.date_baptized),
            SortColumn::Witness => a.witness.cmp(&b.witness),
            SortColumn::Location => a.location.cmp(&b.location),
        };
        match sort.dir {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });
    sorted
}

#[function_component(BaptismsTable)]
pub fn baptisms_table(props: &BaptismsTableProps) -> Html {
    let baptisms = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);

    {
        let baptisms = baptisms.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Baptism>("get_baptisms").await {
                    Ok(data) => {
                        baptisms.set(data);
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

    let sorted_baptisms = sort_baptisms(&*baptisms, &*sort);

    let all_checked = !sorted_baptisms.is_empty()
        && sorted_baptisms
            .iter()
            .all(|b| (*selected).contains(&b.family_id));
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let on_sort_family_id = on_sort!(SortColumn::FamilyId, sort);
    let on_sort_last_name = on_sort!(SortColumn::LastName, sort);
    let on_sort_first_name = on_sort!(SortColumn::FirstName, sort);
    let on_sort_date_baptized = on_sort!(SortColumn::DateBaptized, sort);
    let on_sort_witness = on_sort!(SortColumn::Witness, sort);
    let on_sort_location = on_sort!(SortColumn::Location, sort);

    let on_row_toggle = {
        let selected = selected.clone();
        Callback::from(move |family_id: String| {
            let mut next = (*selected).clone();
            if next.contains(&family_id) {
                next.remove(&family_id);
            } else {
                next.insert(family_id);
            }
            selected.set(next);
        })
    };

    let on_select_all = {
        let sorted_baptisms = sorted_baptisms.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if sorted_baptisms
                .iter()
                .all(|b| (*selected).contains(&b.family_id))
            {
                selected.set(HashSet::new());
            } else {
                selected.set(
                    sorted_baptisms
                        .iter()
                        .map(|b| b.family_id.clone())
                        .collect(),
                );
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let baptisms = baptisms.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let baptisms = baptisms.clone();
            spawn_local(async move {
                match delete_baptisms(to_delete).await {
                    Ok(_) => {
                        let remaining = (*baptisms)
                            .iter()
                            .filter(|b| !(*selected).contains(&b.family_id))
                            .cloned()
                            .collect();
                        baptisms.set(remaining);
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
                    <h3 class="works-table-title">{ "Baptism Records" }</h3>
                    if !sorted_baptisms.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_baptisms.len(),
                                if sorted_baptisms.len() == 1 { "" } else { "s" }) }
                        </span>
                    }
                </div>
                <div class="works-table-toolbar-right">
                    if some_checked {
                        <span class="works-table-selected-label">
                            { format!("{selected_count} selected") }
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
                } else if sorted_baptisms.is_empty() {
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
                        <span class="works-table-empty-sub">{ "Add a baptism record using the form" }</span>
                    </div>
                } else {
                    <table class="works-table">
                        <colgroup>
                            <col class="width: 44px" />
                            <col class="width: 90px" />
                            <col style="width: 130px" />
                            <col style="width: 130px" />
                            <col style="width: 150px" />
                            <col style="width: 200px" />
                            <col style="width: 250px" />
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
                                    onclick={ on_sort_family_id }>
                                    <span class="works-table-th-inner">
                                        { "Family ID" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::FamilyId,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_last_name }>
                                    <span class="works-table-th-inner">
                                        { "Last Name" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::LastName,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_first_name }>
                                    <span class="works-table-th-inner">
                                        { "First Name" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::FirstName,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_date_baptized }>
                                    <span class="works-table-th-inner">
                                        { "Date Baptized" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::DateBaptized,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_witness }>
                                    <span class="works-table-th-inner">
                                        { "Witness" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::Witness,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_location }>
                                    <span class="works-table-th-inner">
                                        { "Location" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::Location,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            { for sorted_baptisms.iter().map(|b| {
                                let family_id = b.family_id.clone();
                                let is_checked = (*selected).contains(&b.family_id);
                                let on_row_toggle = on_row_toggle.clone();
                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ b.family_id.clone() } class={ row_class }>
                                        <td class="works-table-td works-table-td--check">
                                            <input
                                                type="checkbox"
                                                checked={ is_checked }
                                                onchange={
                                                    Callback::from(move |_: Event| {
                                                        on_row_toggle.emit(family_id.clone());
                                                    })
                                                }
                                            />
                                        </td>
                                        <td class="works-table-td">
                                            <span class="works-table-code-badge">
                                                { &b.family_id }
                                            </span>
                                        </td>
                                        <td class="works-table-td">{ &b.last_name }</td>
                                        <td class="works-table-td">{ &b.first_name }</td>
                                        <td class="works-table-td">{ &b.date_baptized }</td>
                                        <td class="works-table-td">{ &b.witness }</td>
                                        <td class="works-table-td">{ &b.location }</td>
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
