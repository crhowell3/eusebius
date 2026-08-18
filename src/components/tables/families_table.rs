use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::icons::{AddRecord, EditBox, ErrorIcon, Eye, Save, TrashCan};
use crate::components::sorting::{SortDir, SortState, sort_icon};
use crate::components::tables::TableMode;
use crate::on_sort;
use crate::utils::{self, fetch_records, format_date, format_phone, invoke};
use models::Family;

async fn delete_families(family_ids: Vec<String>) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        family_ids: Vec<String>,
    }
    let args = to_value(&Args { family_ids }).map_err(|e| e.to_string())?;
    let _ = invoke("delete_families", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

async fn update_family(family: Family) -> Result<(), String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args {
        family: Family,
    }
    let args = to_value(&Args { family }).map_err(|e| e.to_string())?;
    let _ = invoke("update_family", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

#[derive(Default, Clone, PartialEq)]
enum SortColumn {
    #[default]
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

fn sort_families(families: &[Family], sort: &SortState<SortColumn>) -> Vec<Family> {
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

fn cell(value: &str) -> Html {
    if value.is_empty() {
        html! { <span class="cell-empty">{ "—" }</span> }
    } else {
        html! { { value } }
    }
}

#[derive(Properties, PartialEq)]
pub struct FamiliesTableProps {
    pub refresh_trigger: u32,
    pub on_select_family: Callback<String>,
    pub selected_family_id: Option<String>,
}

#[function_component(FamiliesTable)]
pub fn families_table(props: &FamiliesTableProps) -> Html {
    let families = use_state(Vec::<Family>::new);
    let saved_families = use_state(Vec::<Family>::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);
    let mode = use_state(|| TableMode::View);
    let saving = use_state(|| false);

    {
        let families = families.clone();
        let saved_families = saved_families.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_records::<Family>("get_families").await {
                    Ok(data) => {
                        saved_families.set(data.clone());
                        families.set(data);
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

    let sorted_families = sort_families(&*families, &*sort);

    let dirty = *families != *saved_families;
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let is_edit = *mode == TableMode::Edit;

    let all_checked = !sorted_families.is_empty()
        && sorted_families
            .iter()
            .all(|m| (*selected).contains(&m.family_id));

    let on_sort_family_id = on_sort!(SortColumn::FamilyId, sort);
    let on_sort_mail_route = on_sort!(SortColumn::MailRoute, sort);
    let on_sort_last_name = on_sort!(SortColumn::LastName, sort);
    let on_sort_first_name = on_sort!(SortColumn::FirstName, sort);
    let on_sort_date_of_birth = on_sort!(SortColumn::DateOfBirth, sort);
    let on_sort_ann_month = on_sort!(SortColumn::AnniversaryMonth, sort);
    let on_sort_ann_day = on_sort!(SortColumn::AnniversaryDay, sort);
    let on_sort_city = on_sort!(SortColumn::City, sort);
    let on_sort_state = on_sort!(SortColumn::State, sort);

    let on_toggle_mode = {
        let mode = mode.clone();
        let families = families.clone();
        let saved_families = saved_families.clone();
        let selected = selected.clone();

        Callback::from(move |_: MouseEvent| {
            if *mode == TableMode::Edit {
                families.set((*saved_families).clone());
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
        let families = families.clone();
        let saved_families = saved_families.clone();
        let error = error.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let to_delete: Vec<String> = (*selected).iter().cloned().collect();
            let count = to_delete.len();
            let selected = selected.clone();
            let families = families.clone();
            let saved_families = saved_families.clone();
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

                match delete_families(to_delete).await {
                    Ok(_) => {
                        let remaining: Vec<Family> = (*families)
                            .iter()
                            .filter(|m| !(*selected).contains(&m.family_id))
                            .cloned()
                            .collect();
                        families.set(remaining.clone());
                        saved_families.set(remaining);
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
        let families = families.clone();
        let saved_families = saved_families.clone();
        let error = error.clone();
        let saving = saving.clone();
        let mode = mode.clone();

        Callback::from(move |_: MouseEvent| {
            let snapshot = (*saved_families).clone();
            let families = families.clone();

            let changed: Vec<Family> = (*families)
                .iter()
                .filter(|b| {
                    !snapshot.iter().any(|n| {
                        n.family_id == b.family_id
                            && n.mail_route == b.mail_route
                            && n.last_name == b.last_name
                            && n.first_name == b.first_name
                            && n.is_member == b.is_member
                            && n.is_active == b.is_active
                            && n.date_of_birth == b.date_of_birth
                            && n.anniversary_month == b.anniversary_month
                            && n.anniversary_day == b.anniversary_day
                            && n.home_phone == b.home_phone
                            && n.cell_phone == b.cell_phone
                            && n.work_phone == b.work_phone
                            && n.address == b.address
                            && n.city == b.city
                            && n.state == b.state
                            && n.zip == b.zip
                            && n.email_address == b.email_address
                            && n.on_bulletin_email_list == b.on_bulletin_email_list
                    })
                })
                .cloned()
                .collect();

            if changed.is_empty() {
                mode.set(TableMode::View);
                return;
            }

            let families = families.clone();
            let saved_families = saved_families.clone();
            let error = error.clone();
            let saving = saving.clone();
            let mode = mode.clone();

            saving.set(true);

            spawn_local(async move {
                let mut all_ok = true;
                for family in changed {
                    if let Err(e) = update_family(family).await {
                        error.set(Some(e));
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    saved_families.set((*families).clone());
                    error.set(None);
                    mode.set(TableMode::View);
                }

                saving.set(false);
            });
        })
    };

    let on_field_change = {
        let families = families.clone();

        Callback::from(
            move |(family_id, field, value): (String, &'static str, String)| {
                families.set({
                    let mut next = (*families).clone();
                    if let Some(b) = next.iter_mut().find(|b| b.family_id == family_id) {
                        match field {
                            "family_id" => b.family_id = value,
                            "mail_route" => b.mail_route = value,
                            "last_name" => b.last_name = value,
                            "first_name" => b.first_name = value,
                            "date_of_birth" => b.date_of_birth = value,
                            "anniversary_month" => b.anniversary_month = value,
                            "anniversary_day" => b.anniversary_day = value,
                            "home_phone" => b.home_phone = value,
                            "cell_phone" => b.cell_phone = value,
                            "work_phone" => b.work_phone = value,
                            "address" => b.address = value,
                            "city" => b.city = value,
                            "state" => b.state = value,
                            "zip" => b.zip = value,
                            "email_address" => b.email_address = value,
                            _ => unreachable!(),
                        }
                    }
                    next
                })
            },
        )
    };

    let on_bool_change = {
        let families = families.clone();

        Callback::from(
            move |(family_id, field, value): (String, &'static str, bool)| {
                families.set({
                    let mut next = (*families).clone();
                    if let Some(f) = next.iter_mut().find(|f| f.family_id == family_id) {
                        match field {
                            "is_member" => f.is_member = value,
                            "is_active" => f.is_active = value,
                            "on_bulletin_email_list" => f.on_bulletin_email_list = value,
                            _ => unreachable!(),
                        }
                    }
                    next
                })
            },
        )
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
                } else if sorted_families.is_empty() {
                    <div class="works-table-empty">
                        <AddRecord />
                        <span class="works-table-empty-text">{ "No records yet" }</span>
                        <span class="works-table-empty-sub">{ "Add a record using the form" }</span>
                    </div>
                } else {
                    <table class="family-table">
                        <colgroup>
                            { if is_edit { html! { <col style="width: 44px" /> } } else { html! {} } }
                            <col style="width: 90px" />
                            <col style="width: 90px" />
                            <col style="width: 130px" />
                            <col style="width: 130px" />
                            <col style="width: 60px" />
                            <col style="width: 60px" />
                            <col style="width: 110px" />
                            <col style="width: 100px" />
                            <col style="width: 70px" />
                            <col style="width: 170px" />
                            <col style="width: 170px" />
                            <col style="width: 170px" />
                            <col style="width: 180px" />
                            <col style="width: 110px" />
                            <col style="width: 60px" />
                            <col style="width: 70px" />
                            <col style="width: 180px" />
                            <col style="width: 80px" />
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
                                <th class="works-table-th">{ "Home Phone" }</th>
                                <th class="works-table-th">{ "Cell Phone" }</th>
                                <th class="works-table-th">{ "Work Phone" }</th>
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

                                let on_change_mail_route = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "mail_route");
                                let on_change_last_name = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "last_name");
                                let on_change_first_name = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "first_name");
                                let on_change_date_of_birth = utils::callback_factories::make_formatted_callback(on_field_change.clone(), family_id.clone(), "date_of_birth", format_date);
                                let on_change_anniversary_month = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "anniversary_month");
                                let on_change_anniversary_day = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "anniversary_day");
                                let on_change_home_phone = utils::callback_factories::make_formatted_callback(on_field_change.clone(), family_id.clone(), "home_phone", format_phone);
                                let on_change_cell_phone = utils::callback_factories::make_formatted_callback(on_field_change.clone(), family_id.clone(), "cell_phone", format_phone);
                                let on_change_work_phone = utils::callback_factories::make_formatted_callback(on_field_change.clone(), family_id.clone(), "work_phone", format_phone);
                                let on_change_address = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "address");
                                let on_change_city = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "city");
                                let on_change_state = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "state");
                                let on_change_zip = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "zip");
                                let on_change_email_address = utils::callback_factories::make_field_callback(on_field_change.clone(), family_id.clone(), "email_address");

                                let on_change_is_member = utils::callback_factories::make_bool_callback(on_bool_change.clone(), family_id.clone(), "is_member");
                                let on_change_is_active = utils::callback_factories::make_bool_callback(on_bool_change.clone(), family_id.clone(), "is_active");
                                let on_change_on_bulletin_email_list = utils::callback_factories::make_bool_callback(on_bool_change.clone(), family_id.clone(), "on_bulletin_email_list");

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
                                                { &m.family_id }
                                            </span>
                                        </td>
                                        {
                                            if is_edit {
                                                html! {
                                                    <>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                max_length="3"
                                                                inputmode="numeric"
                                                                value={ m.mail_route.clone() }
                                                                oninput={ on_change_mail_route }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                value={ m.last_name.clone() }
                                                                oninput={ on_change_last_name }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                value={ m.first_name.clone() }
                                                                oninput={ on_change_first_name }
                                                            />
                                                        </td>
                                                        <td class="works-table-td">
                                                            <input type="checkbox" id="is_member" name="is_member"
                                                                class="form-checkbox-input"
                                                                checked={ m.is_member }
                                                                onchange={ on_change_is_member }
                                                            />
                                                        </td>
                                                        <td class="works-table-td">
                                                            <input type="checkbox" id="is_active" name="is_active"
                                                                class="form-checkbox-input"
                                                                checked={ m.is_active }
                                                                onchange={ on_change_is_active }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                max_length="10"
                                                                inputmode="numeric"
                                                                value={ m.date_of_birth.clone() }
                                                                oninput={ on_change_date_of_birth }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                value={ m.anniversary_month.clone() }
                                                                oninput={ on_change_anniversary_month }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                value={ m.anniversary_day.clone() }
                                                                oninput={ on_change_anniversary_day }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                max_length="14"
                                                                inputmode="numeric"
                                                                value={ m.home_phone.clone() }
                                                                oninput={ on_change_home_phone }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                max_length="14"
                                                                inputmode="numeric"
                                                                value={ m.cell_phone.clone() }
                                                                oninput={ on_change_cell_phone }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                max_length="14"
                                                                inputmode="numeric"
                                                                value={ m.work_phone.clone() }
                                                                oninput={ on_change_work_phone }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                value={ m.address.clone() }
                                                                oninput={ on_change_address }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                value={ m.city.clone() }
                                                                oninput={ on_change_city }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                value={ m.state.clone() }
                                                                oninput={ on_change_state }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                value={ m.zip.clone() }
                                                                oninput={ on_change_zip }
                                                            />
                                                        </td>
                                                        <td class="works-table-td family-td-clip">
                                                            <input
                                                                type="text"
                                                                class="cell-input"
                                                                placeholder="—"
                                                                value={ m.email_address.clone() }
                                                                oninput={ on_change_email_address }
                                                            />
                                                        </td>
                                                        <td class="works-table-td">
                                                            <input type="checkbox" id="is_active" name="is_active"
                                                                class="form-checkbox-input"
                                                                checked={ m.on_bulletin_email_list }
                                                                onchange={ on_change_on_bulletin_email_list }
                                                            />
                                                        </td>
                                                    </>
                                                }
                                            } else {
                                                html! {
                                                    <>
                                                        <td class="works-table-td family-td-clip">{ &m.mail_route }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.last_name }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.first_name }</td>
                                                        <td class="works-table-td">{ bool_cell(m.is_member) }</td>
                                                        <td class="works-table-td">{ bool_cell(m.is_active) }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.date_of_birth }</td>
                                                        <td class="works-table-td family-td-clip">{ cell(&m.anniversary_month) }</td>
                                                        <td class="works-table-td family-td-clip">{ cell(&m.anniversary_day) }</td>
                                                        <td class="works-table-td family-td-clip">{ cell(&m.home_phone) }</td>
                                                        <td class="works-table-td family-td-clip">{ cell(&m.cell_phone) }</td>
                                                        <td class="works-table-td family-td-clip">{ cell(&m.work_phone) }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.address }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.city }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.state }</td>
                                                        <td class="works-table-td family-td-clip">{ &m.zip }</td>
                                                        <td class="works-table-td family-td-clip">{ cell(&m.email_address) }</td>
                                                        <td class="works-table-td">{ bool_cell(m.on_bulletin_email_list) }</td>
                                                    </>
                                                } } }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_cell() {
        let value = "";
        let expected_data = html! { <span class="cell-empty">{ "—" }</span> };

        assert_eq!(cell(value), expected_data);
    }

    #[test]
    fn test_populated_cell() {
        let value = "some_data";
        let expected_data = html! { { value } };

        assert_eq!(cell(value), expected_data);
    }
}
