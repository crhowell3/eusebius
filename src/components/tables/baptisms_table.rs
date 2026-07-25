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
use models::Baptism;

async fn delete_baptisms(family_ids: Vec<String>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        family_ids: Vec<String>,
    }
    let args = to_value(&Args { family_ids }).map_err(|e| e.to_string())?;
    let _ = invoke("delete_baptisms", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

async fn update_baptism(baptism: Baptism) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        baptism: Baptism,
    }
    let args = to_value(&Args { baptism }).map_err(|e| e.to_string())?;
    let _ = invoke("update_baptism", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
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

#[derive(Properties, PartialEq)]
pub struct BaptismsTableProps {
    pub refresh_trigger: u32,
}

#[function_component(BaptismsTable)]
pub fn baptisms_table(props: &BaptismsTableProps) -> Html {
    let baptisms = use_state(Vec::<Baptism>::new);
    let saved_baptisms = use_state(Vec::<Baptism>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);

    {
        let baptisms = baptisms.clone();
        let saved_baptisms = saved_baptisms.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Baptism>("get_baptisms").await {
                    Ok(data) => {
                        saved_baptisms.set(data.clone());
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

    let dirty = *baptisms != *saved_baptisms;
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let all_checked = !sorted_baptisms.is_empty()
        && sorted_baptisms
            .iter()
            .all(|b| (*selected).contains(&b.family_id));

    let on_sort_family_id = on_sort!(SortColumn::FamilyId, sort);
    let on_sort_last_name = on_sort!(SortColumn::LastName, sort);
    let on_sort_first_name = on_sort!(SortColumn::FirstName, sort);
    let on_sort_date_baptized = on_sort!(SortColumn::DateBaptized, sort);
    let on_sort_witness = on_sort!(SortColumn::Witness, sort);
    let on_sort_location = on_sort!(SortColumn::Location, sort);

    let on_toggle_mode = {
        let mode = mode.clone();
        let baptisms = baptisms.clone();
        let saved_baptisms = saved_baptisms.clone();
        let selected = selected.clone();

        Callback::from(move |_: MouseEvent| {
            if *mode == TableMode::Edit {
                baptisms.set((*saved_baptisms).clone());
                selected.set(HashSet::new());
                mode.set(TableMode::View);
            } else {
                mode.set(TableMode::Edit);
            }
        })
    };

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
        let saved_baptisms = saved_baptisms.clone();
        let error = error.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let count = to_delete.len();
            let selected = selected.clone();
            let baptisms = baptisms.clone();
            let saved_baptisms = saved_baptisms.clone();
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
                        "Delete {count} selected baptism{}? This cannot be undone.",
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

                match delete_baptisms(to_delete).await {
                    Ok(_) => {
                        let remaining: Vec<Baptism> = (*baptisms)
                            .iter()
                            .filter(|b| !(*selected).contains(&b.family_id))
                            .cloned()
                            .collect();
                        baptisms.set(remaining.clone());
                        saved_baptisms.set(remaining);
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
        let baptisms = baptisms.clone();
        let saved_baptisms = saved_baptisms.clone();
        let error = error.clone();
        let saving = saving.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let snapshot = (*saved_baptisms).clone();
            let baptisms = baptisms.clone();

            let changed: Vec<Baptism> = (*baptisms)
                .iter()
                .filter(|b| {
                    !snapshot.iter().any(|n| {
                        n.family_id == b.family_id
                            && n.first_name == b.first_name
                            && n.last_name == b.last_name
                            && n.date_baptized == b.date_baptized
                            && n.location == b.location
                            && n.witness == b.witness
                    })
                })
                .cloned()
                .collect();

            if changed.is_empty() {
                mode.set(TableMode::View);
                return;
            }

            let baptisms = baptisms.clone();
            let saved_baptisms = saved_baptisms.clone();
            let error = error.clone();
            let saving = saving.clone();
            let mode = mode.clone();

            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for baptism in changed {
                    if let Err(e) = update_baptism(baptism).await {
                        error.set(Some(e));
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    saved_baptisms.set((*baptisms).clone());
                    error.set(None);
                    mode.set(TableMode::View);
                }

                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let baptisms = baptisms.clone();

        Callback::from(
            move |(family_id, field, value): (String, &'static str, String)| {
                baptisms.set({
                    let mut next = (*baptisms).clone();
                    if let Some(b) = next.iter_mut().find(|b| b.family_id == family_id) {
                        match field {
                            "family_id" => b.family_id = value,
                            "last_name" => b.last_name = value,
                            "first_name" => b.first_name = value,
                            "date_baptized" => b.date_baptized = value,
                            "witness" => b.witness = value,
                            "location" => b.location = value,
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
                    <h3 class="works-table-title">{ "Baptisms" }</h3>
                    if !sorted_baptisms.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_baptisms.len(),
                                if sorted_baptisms.len() == 1 { "" } else { "s" }) }
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
                } else if sorted_baptisms.is_empty() {
                    <div class="works-table-empty">
                        <AddRecord />
                        <span class="works-table-empty-text">{ "No records yet" }</span>
                        <span class="works-table-empty-sub">{ "Add a baptism record using the form" }</span>
                    </div>
                } else {
                    <table class="works-table">
                        <colgroup>
                            { if is_edit { html! { <col style="width: 44px" /> } } else { html! {} } }
                            <col style="width: 90px" />
                            <col style="width: 130px" />
                            <col style="width: 130px" />
                            <col style="width: 150px" />
                            <col style="width: 200px" />
                            <col style="width: 250px" />
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
                                let on_field_change = on_field_change.clone();

                                let make_field_callback = {
                                    let on_field_change = on_field_change.clone();
                                    let family_id = family_id.clone();
                                    move |field_name: &'static str| {
                                        let on_field_change = on_field_change.clone();
                                        let family_id = family_id.clone();
                                        Callback::from(move |e: InputEvent| {
                                            use wasm_bindgen::JsCast;
                                            use web_sys::HtmlInputElement;
                                            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                                                on_field_change.emit((family_id.clone(), field_name, input.value()));
                                            }
                                        })
                                    }
                                };

                                let on_change_last_name = make_field_callback("last_name");
                                let on_change_first_name = make_field_callback("first_name");
                                let on_change_date_baptized = make_field_callback("date_baptized");
                                let on_change_witness = make_field_callback("witness");
                                let on_change_location = make_field_callback("location");

                                let row_class = if is_checked {
                                    "works-table-row works-table-row--selected"
                                } else {
                                    "works-table-row"
                                };

                                html! {
                                    <tr key={ b.family_id.clone() } class={ row_class }>
                                        { if is_edit { html! {
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
                                        } } else { html! {} } }
                                        <td class="works-table-td">
                                            <span class="works-table-code-badge">
                                                { &b.family_id }
                                            </span>
                                        </td>
                                        <td class="works-table-td">
                                            {
                                                if is_edit {
                                                    html! {
                                                        <input
                                                            type="text"
                                                            class="cell-input"
                                                            value={ b.last_name.clone() }
                                                            oninput={ on_change_last_name }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &b.last_name }
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
                                                            value={ b.first_name.clone() }
                                                            oninput={ on_change_first_name }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &b.first_name }
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
                                                            value={ b.date_baptized.clone() }
                                                            oninput={ on_change_date_baptized }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &b.date_baptized }
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
                                                            value={ b.witness.clone() }
                                                            oninput={ on_change_witness }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &b.witness }
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
                                                            value={ b.location.clone() }
                                                            oninput={ on_change_location }
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        { &b.location }
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
