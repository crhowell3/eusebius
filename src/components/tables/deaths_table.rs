use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::sorting::{SortDir, SortState, sort_icon};
use crate::on_sort;
use crate::utils::{fetch_records, invoke};
use models::Death;

#[derive(Properties, PartialEq)]
pub struct DeathsTableProps {
    pub refresh_trigger: u32,
}

async fn delete_deaths(death_ids: Vec<i64>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        death_ids: Vec<i64>,
    }
    let args = to_value(&Args { death_ids }).map_err(|e| e.to_string())?;
    let result = invoke("delete_deaths", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Default, Clone, PartialEq)]
enum SortColumn {
    #[default]
    FirstName,
    LastName,
    DateOfDeath,
}

fn sort_deaths(deaths: &[Death], sort: &SortState<SortColumn>) -> Vec<Death> {
    let mut sorted = deaths.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::FirstName => a.first_name.cmp(&b.first_name),
            SortColumn::LastName => a.last_name.cmp(&b.last_name),
            SortColumn::DateOfDeath => a.date_of_death.cmp(&b.date_of_death),
        };
        match sort.dir {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });
    sorted
}

#[function_component(DeathsTable)]
pub fn deaths_table(props: &DeathsTableProps) -> Html {
    let deaths = use_state(Vec::<Death>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);

    {
        let deaths = deaths.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Death>("get_deaths").await {
                    Ok(data) => {
                        deaths.set(data);
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

    let sorted_deaths = sort_deaths(&*deaths, &*sort);

    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let all_checked =
        !(*deaths).is_empty() && (*deaths).iter().all(|c| (*selected).contains(&c.id));

    let on_sort_first_name = on_sort!(SortColumn::FirstName, sort);
    let on_sort_last_name = on_sort!(SortColumn::LastName, sort);
    let on_sort_death_date = on_sort!(SortColumn::DateOfDeath, sort);

    let on_row_toggle = {
        let selected = selected.clone();
        Callback::from(move |death_id: i64| {
            let mut next = (*selected).clone();
            if next.contains(&death_id) {
                next.remove(&death_id);
            } else {
                next.insert(death_id);
            }
            selected.set(next);
        })
    };

    let on_select_all = {
        let sorted_deaths = sorted_deaths.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if sorted_deaths.iter().all(|w| (*selected).contains(&w.id)) {
                selected.set(HashSet::new());
            } else {
                selected.set(sorted_deaths.iter().map(|w| w.id.clone()).collect());
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let deaths = deaths.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let deaths = deaths.clone();
            spawn_local(async move {
                match delete_deaths(to_delete).await {
                    Ok(_) => {
                        let remaining = (*deaths)
                            .iter()
                            .filter(|w| !(*selected).contains(&w.id))
                            .cloned()
                            .collect();
                        deaths.set(remaining);
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
                    <h3 class="works-table-title">{ "Death Records" }</h3>
                    if !sorted_deaths.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_deaths.len(),
                                if sorted_deaths.len() == 1 { "" } else { "s" }) }
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
                } else if sorted_deaths.is_empty() {
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
                        <span class="works-table-empty-sub">{ "Add a record using the form" }</span>
                    </div>
                } else {
                    <table class="family-table">
                        <colgroup>
                            <col style="width: 44px" />
                            <col style="width: 130px" />  // first name
                            <col style="width: 130px" />  // last name
                            <col style="width: 110px" />  // dod
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
                                    onclick={ on_sort_death_date }>
                                    <span class="works-table-th-inner">
                                        { "Date of Death" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::DateOfDeath,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                        { for sorted_deaths.iter().enumerate().map(|(_, c)| {
                                let death_id = c.id;
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
                                                    on_row_toggle.emit(death_id);
                                                }) }
                                            />
                                        </td>
                                        <td class="works-table-td">{ &c.first_name }</td>
                                        <td class="works-table-td">{ &c.last_name }</td>
                                        <td class="works-table-td">{ &c.date_of_death }</td>
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
