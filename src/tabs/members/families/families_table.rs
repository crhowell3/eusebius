use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;
use shared::Family;

#[derive(Properties, PartialEq)]
pub struct FamiliesTableProps {
    pub refresh_trigger: u32,
    pub on_select_family: Callback<String>,
    pub selected_family_id: Option<String>,
}

async fn fetch_families() -> Result<Vec<Family>, String> {
    let result = invoke("get_families", JsValue::UNDEFINED).await;
    from_value::<Vec<Family>>(result).map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteFamiliesArgs {
    family_ids: Vec<String>,
}

async fn delete_families(family_ids: Vec<String>) -> Result<(), String> {
    let args = to_value(&DeleteFamiliesArgs { family_ids }).map_err(|e| e.to_string())?;
    let result = invoke("delete_families", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Clone, PartialEq)]
enum SortColumn {
    FamilyId,
    MailRoute,
    LastName,
    FirstName,
    DateOfBirth,
    AnniversaryMonth,
    AnniversaryDay,
    City,
    State,
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
            column: SortColumn::FamilyId,
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

fn sort_families(families: &[Family], sort: &SortState) -> Vec<Family> {
    let mut sorted = families.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::FamilyId => a.family_id.cmp(&b.family_id),
            SortColumn::MailRoute => a.mail_route.cmp(&b.mail_route),
            SortColumn::LastName => a.last_name.cmp(&b.last_name),
            SortColumn::FirstName => a.first_name.cmp(&b.first_name),
            SortColumn::DateOfBirth => a.date_of_birth.cmp(&b.date_of_birth),
            SortColumn::AnniversaryMonth => a.anniversary_month.cmp(&b.anniversary_month),
            SortColumn::AnniversaryDay => a.anniversary_day.cmp(&b.anniversary_day),
            SortColumn::City => a.city.cmp(&b.city),
            SortColumn::State => a.state.cmp(&b.state),
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

fn bool_cell(value: bool) -> Html {
    if value {
        html! {
            <span class="family-badge family-badge--yes">{ "Yes" }</span>
        }
    } else {
        html! {
            <span class="family-badge family-badge--no">{ "No" }</span>
        }
    }
}

#[function_component(FamiliesTable)]
pub fn families_table(props: &FamiliesTableProps) -> Html {
    let members = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);

    {
        let members = members.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_families().await {
                    Ok(data) => {
                        members.set(data);
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

    let sorted_families = sort_families(&*members, &*sort);

    let all_checked = !sorted_families.is_empty()
        && sorted_families
            .iter()
            .all(|m| (*selected).contains(&m.family_id));
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    macro_rules! on_sort {
        ($col:expr) => {{
            let sort = sort.clone();
            Callback::from(move |_: MouseEvent| sort.set((*sort).clone().toggle($col)))
        }};
    }

    let on_sort_family_id = on_sort!(SortColumn::FamilyId);
    let on_sort_mail_route = on_sort!(SortColumn::MailRoute);
    let on_sort_last_name = on_sort!(SortColumn::LastName);
    let on_sort_first_name = on_sort!(SortColumn::FirstName);
    let on_sort_date_of_birth = on_sort!(SortColumn::DateOfBirth);
    let on_sort_ann_month = on_sort!(SortColumn::AnniversaryMonth);
    let on_sort_ann_day = on_sort!(SortColumn::AnniversaryDay);
    let on_sort_city = on_sort!(SortColumn::City);
    let on_sort_state = on_sort!(SortColumn::State);

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
        let sorted_families = sorted_families.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            if sorted_families
                .iter()
                .all(|m| (*selected).contains(&m.family_id))
            {
                selected.set(HashSet::new());
            } else {
                selected.set(
                    sorted_families
                        .iter()
                        .map(|m| m.family_id.clone())
                        .collect(),
                );
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let members = members.clone();
        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let selected = selected.clone();
            let members = members.clone();
            spawn_local(async move {
                match delete_families(to_delete).await {
                    Ok(_) => {
                        let remaining = (*members)
                            .iter()
                            .filter(|m| !(*selected).contains(&m.family_id))
                            .cloned()
                            .collect();
                        members.set(remaining);
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
                    <h3 class="works-table-title">{ "Family Records" }</h3>
                    if !sorted_families.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_families.len(),
                                if sorted_families.len() == 1 { "" } else { "s" }) }
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
                } else if sorted_families.is_empty() {
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
                            <col style="width: 90px" />
                            <col style="width: 90px" />
                            <col style="width: 130px" />
                            <col style="width: 130px" />
                            <col style="width: 60px" />
                            <col style="width: 60px" />
                            <col style="width: 110px" />
                            <col style="width: 100px" />
                            <col style="width: 70px" />
                            <col style="width: 120px" />
                            <col style="width: 120px" />
                            <col style="width: 120px" />
                            <col style="width: 180px" />
                            <col style="width: 110px" />
                            <col style="width: 60px" />
                            <col style="width: 70px" />
                            <col style="width: 180px" />
                            <col style="width: 80px" />
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
                                        { "Family Id" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::FamilyId,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_mail_route }>
                                    <span class="works-table-th-inner">
                                        { "Mail Route" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::MailRoute,
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
                                <th class="works-table-th">{ "Mbr?" }</th>
                                <th class="works-table-th">{ "Active?" }</th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_date_of_birth }>
                                    <span class="works-table-th-inner">
                                        { "DOB" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::DateOfBirth,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_ann_month }>
                                    <span class="works-table-th-inner">
                                        { "Ann. Month" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::AnniversaryMonth,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_ann_day }>
                                    <span class="works-table-th-inner">
                                        { "Ann. Day" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::AnniversaryDay,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th">{ "Home Ph." }</th>
                                <th class="works-table-th">{ "Cell Ph." }</th>
                                <th class="works-table-th">{ "Work Ph." }</th>
                                <th class="works-table-th">{ "Address" }</th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_city }>
                                    <span class="works-table-th-inner">
                                        { "City" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::City,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th works-table-th--sortable"
                                    onclick={ on_sort_state }>
                                    <span class="works-table-th-inner">
                                        { "State" }
                                        { sort_icon(
                                            (*sort).column == SortColumn::State,
                                            &(*sort).dir
                                        ) }
                                    </span>
                                </th>
                                <th class="works-table-th">{ "ZIP" }</th>
                                <th class="works-table-th">{ "Email" }</th>
                                <th class="works-table-th">{ "Bulletin?" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for sorted_families.iter().map(|m| {
                                let family_id = m.family_id.clone();
                                let is_checked = (*selected).contains(&m.family_id);

                                let is_active_family = props.selected_family_id.as_deref() == Some(m.family_id.as_str());

                                let on_row_toggle = on_row_toggle.clone();

                                let on_dbl_click = {
                                    let on_select_family = props.on_select_family.clone();
                                    let family_id = m.family_id.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        on_select_family.emit(family_id.clone());
                                    })
                                };
                                let row_class = match (is_checked, is_active_family) {
                                    (true, _)     => "works-table-row works-table-row--selected",
                                    (false, true) => "works-table-row works-table-row--active-family",
                                    (false, false) => "works-table-row",
                                };

                                html! {
                                    <tr key={ m.family_id.clone() } class={ row_class } ondblclick={ on_dbl_click }>
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
                                                { &m.family_id }
                                            </span>
                                        </td>
                                        <td class="works-table-td family-td-clip">{ &m.mail_route }</td>
                                        <td class="works-table-td family-td-clip">{ &m.last_name }</td>
                                        <td class="works-table-td family-td-clip">{ &m.first_name }</td>
                                        <td class="works-table-td">{ bool_cell(m.is_member) }</td>
                                        <td class="works-table-td">{ bool_cell(m.is_active) }</td>
                                        <td class="works-table-td family-td-clip">{ &m.date_of_birth }</td>
                                        <td class="works-table-td family-td-clip">{ &m.anniversary_month }</td>
                                        <td class="works-table-td family-td-clip">{ &m.anniversary_day }</td>
                                        <td class="works-table-td family-td-clip">{ &m.home_phone }</td>
                                        <td class="works-table-td family-td-clip">{ &m.cell_phone }</td>
                                        <td class="works-table-td family-td-clip">{ &m.work_phone }</td>
                                        <td class="works-table-td family-td-clip">{ &m.address }</td>
                                        <td class="works-table-td family-td-clip">{ &m.city }</td>
                                        <td class="works-table-td family-td-clip">{ &m.state }</td>
                                        <td class="works-table-td family-td-clip">{ &m.zip }</td>
                                        <td class="works-table-td family-td-clip">{ &m.email_address }</td>
                                        <td class="works-table-td">{ bool_cell(m.on_bulletin_email_list) }</td>
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
