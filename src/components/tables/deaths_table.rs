use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::icons::{AddRecord, EditBox, ErrorIcon, Eye, Save, TrashCan};
use crate::components::sorting::{SortDir, SortState, sort_icon};
use crate::components::tables::TableMode;
use crate::on_sort;
use crate::utils::{fetch_records, invoke};
use models::Death;

async fn delete_deaths(death_ids: Vec<i64>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        death_ids: Vec<i64>,
    }
    let args = to_value(&Args { death_ids }).map_err(|e| e.to_string())?;
    let _ = invoke("delete_deaths", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

async fn update_death(death: Death) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        death: Death,
    }
    let args = to_value(&Args { death }).map_err(|e| e.to_string())?;
    let _ = invoke("update_death", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
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

#[derive(Properties, PartialEq)]
pub struct DeathsTableProps {
    pub refresh_trigger: u32,
}

#[function_component(DeathsTable)]
pub fn deaths_table(props: &DeathsTableProps) -> Html {
    let deaths = use_state(Vec::<Death>::new);
    let saved_deaths = use_state(Vec::<Death>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<i64>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);

    {
        let deaths = deaths.clone();
        let saved_deaths = saved_deaths.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Death>("get_deaths").await {
                    Ok(data) => {
                        saved_deaths.set(data.clone());
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

    let dirty = *deaths != *saved_deaths;
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let all_checked =
        !(*deaths).is_empty() && (*deaths).iter().all(|c| (*selected).contains(&c.id));

    let on_sort_first_name = on_sort!(SortColumn::FirstName, sort);
    let on_sort_last_name = on_sort!(SortColumn::LastName, sort);
    let on_sort_death_date = on_sort!(SortColumn::DateOfDeath, sort);

    let on_toggle_mode = {
        let mode = mode.clone();
        let deaths = deaths.clone();
        let saved_deaths = saved_deaths.clone();
        let selected = selected.clone();

        Callback::from(move |_: MouseEvent| {
            if *mode == TableMode::Edit {
                deaths.set((*saved_deaths).clone());
                selected.set(HashSet::new());
                mode.set(TableMode::View);
            } else {
                mode.set(TableMode::Edit);
            }
        })
    };

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
        let saved_deaths = saved_deaths.clone();
        let error = error.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<i64> = (*selected).iter().cloned().collect();
            let count = to_delete.len();
            let selected = selected.clone();
            let deaths = deaths.clone();
            let saved_deaths = saved_deaths.clone();
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
                        "Delete {count} selected death record{}? This cannot be undone.",
                        if count == 1 { "" } else { "s" }
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

                match delete_deaths(to_delete).await {
                    Ok(_) => {
                        let remaining: Vec<Death> = (*deaths)
                            .iter()
                            .filter(|w| !(*selected).contains(&w.id))
                            .cloned()
                            .collect();
                        deaths.set(remaining.clone());
                        saved_deaths.set(remaining);
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
        let deaths = deaths.clone();
        let saved_deaths = saved_deaths.clone();
        let error = error.clone();
        let saving = saving.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let snapshot = (*saved_deaths).clone();
            let deaths = deaths.clone();

            let changed: Vec<Death> = (*deaths)
                .iter()
                .filter(|d| {
                    !snapshot.iter().any(|n| {
                        n.first_name == d.first_name
                            && n.last_name == d.last_name
                            && n.date_of_death == d.date_of_death
                    })
                })
                .cloned()
                .collect();

            if changed.is_empty() {
                mode.set(TableMode::View);
                return;
            }

            let deaths = deaths.clone();
            let saved_deaths = saved_deaths.clone();
            let error = error.clone();
            let saving = saving.clone();
            let mode = mode.clone();

            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for death in changed {
                    if let Err(e) = update_death(death).await {
                        error.set(Some(e));
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    saved_deaths.set((*deaths).clone());
                    error.set(None);
                    mode.set(TableMode::View);
                }

                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let deaths = deaths.clone();

        Callback::from(
            move |(death_id, field, value): (i64, &'static str, String)| {
                deaths.set({
                    let mut next = (*deaths).clone();
                    if let Some(d) = next.iter_mut().find(|d| d.id == death_id) {
                        match field {
                            "first_name" => d.first_name = value,
                            "last_name" => d.last_name = value,
                            "date_of_death" => d.date_of_death = value,
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    next
                });
            },
        )
    };

    html! {
        <section class="works-table-card">
            <div class="works-table-toolbar">
                <div class="works-table-toolbar-left">
                    <h3 class="works-table-title">{ "Deaths" }</h3>
                    if !sorted_deaths.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_deaths.len(),
                                if sorted_deaths.len() == 1 { "" } else { "s" }) }
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
                if *initial_load {
                    <div class="works-table-empty">
                        <span class="works-table-empty-text">{ "Loading..." }</span>
                    </div>
                } else if sorted_deaths.is_empty() {
                    <div class="works-table-empty">
                        <AddRecord />
                        <span class="works-table-empty-text">{ "No records yet" }</span>
                        <span class="works-table-empty-sub">{ "Add a record using the form" }</span>
                    </div>
                } else {
                    <table class="works-table">
                        <colgroup>
                            { if is_edit { html! { <col style="width: 44px" /> } } else { html! {} } }
                            <col style="width: 130px" />  // first name
                            <col style="width: 130px" />  // last name
                            <col style="width: 110px" />  // dod
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
                        { for sorted_deaths.iter().map(|d| {
                                let death_id = d.id;
                                let is_checked = (*selected).contains(&d.id);
                                let on_row_toggle = on_row_toggle.clone();
                                let on_field_change = on_field_change.clone();

                                let make_field_callback = {
                                    let on_field_change = on_field_change.clone();
                                    let death_id = death_id.clone();
                                    move |field_name: &'static str| {
                                        let on_field_change = on_field_change.clone();
                                        let death_id = death_id.clone();
                                        Callback::from(move |e: InputEvent| {
                                            use wasm_bindgen::JsCast;
                                            use web_sys::HtmlInputElement;
                                            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                                on_field_change.emit((death_id.clone(), field_name, input.value()));
                                            }
                                        })
                                    }
                                };

                                let on_change_first_name = make_field_callback("first_name");
                                let on_change_last_name = make_field_callback("last_name");
                                let on_change_date_of_death = make_field_callback("date_of_death");

                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ d.id } class={ row_class }>
                                        { if is_edit { html! {
                                            <td class="works-table-td works-table-td--check">
                                                <input
                                                    type="checkbox"
                                                    checked={ is_checked }
                                                    onchange={
                                                        Callback::from(move |_: Event| {
                                                            on_row_toggle.emit(death_id);
                                                        })
                                                    }
                                                />
                                            </td>
                                        } } else { html! {} } }
                                        <td class="works-table-td">
                                            {
                                                if is_edit {
                                                    html! {
                                                        <input
                                                            type="text"
                                                            class="cell-input"
                                                            value={ d.first_name.clone() }
                                                            oninput={ on_change_first_name }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &d.first_name }
                                                    }
                                                }
                                            }
                                        </td>
                                        <td class="works-table-td">
                                            {
                                                if is_edit {
                                                    html! {
                                                        <input
                                                            type="text"
                                                            class="cell-input"
                                                            value={ d.last_name.clone() }
                                                            oninput={ on_change_last_name }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &d.last_name }
                                                    }
                                                }
                                            }
                                        </td>
                                        <td class="works-table-td">
                                            {
                                                if is_edit {
                                                    html! {
                                                        <input
                                                            type="text"
                                                            class="cell-input"
                                                            value={ d.date_of_death.clone() }
                                                            oninput={ on_change_date_of_death }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &d.date_of_death }
                                                    }
                                                }
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
