use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::invoke;
use shared::{Spouse, SpouseAction};

// ── Invoke wrappers ───────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetSpouseArgs {
    family_id: String,
}

async fn fetch_spouse(family_id: &str) -> Result<Option<Spouse>, String> {
    let args = to_value(&GetSpouseArgs {
        family_id: family_id.to_string(),
    })
    .map_err(|e| e.to_string())?;
    let result = invoke("get_spouse", args).await;
    from_value::<Option<Spouse>>(result).map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct SaveSpouseArgs {
    spouse: Spouse,
}

async fn save_spouse(spouse: Spouse) -> Result<(), String> {
    let args = to_value(&SaveSpouseArgs { spouse }).map_err(|e| e.to_string())?;
    let result = invoke("save_spouse", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

// ── Props ─────────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct SpousesProps {
    pub selected_family_id: Option<String>,
}

// ── Component ─────────────────────────────────────────────────────────────────

#[function_component(Spouses)]
pub fn spouses(props: &SpousesProps) -> Html {
    let spouse = use_reducer(Spouse::default);
    let loading = use_state(|| false);
    let saving = use_state(|| false);
    let dirty = use_state(|| false);
    let error = use_state(|| None::<String>);
    let is_existing = use_state(|| false); // true = record came from DB, false = new

    // ── Fetch spouse when selected family changes ─────────────────────────────
    {
        let spouse = spouse.dispatcher();
        let loading = loading.clone();
        let dirty = dirty.clone();
        let error = error.clone();
        let is_existing = is_existing.clone();
        let family_id = props.selected_family_id.clone();

        use_effect_with(family_id.clone(), move |family_id| {
            match family_id.clone() {
                Some(fid) => {
                    loading.set(true);
                    spawn_local(async move {
                        match fetch_spouse(&fid).await {
                            Ok(Some(existing)) => {
                                // Pre-fill the form with the existing record
                                spouse.dispatch(SpouseAction::Load(existing));
                                is_existing.set(true);
                            }
                            Ok(None) => {
                                // No record — reset to blank but set family_id
                                spouse.dispatch(SpouseAction::Reset);
                                spouse.dispatch(SpouseAction::SetField {
                                    name: "family_id".to_string(),
                                    value: fid,
                                });
                                is_existing.set(false);
                            }
                            Err(e) => error.set(Some(e)),
                        }
                        dirty.set(false);
                        loading.set(false);
                    });
                }
                None => {
                    spouse.dispatch(SpouseAction::Reset);
                    dirty.set(false);
                    is_existing.set(false);
                }
            }
            || ()
        });
    }

    // ── Change handler ────────────────────────────────────────────────────────
    let handle_spouse_change = {
        let spouse = spouse.dispatcher();
        let dirty = dirty.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let (name, value) = if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else {
                return;
            };
            spouse.dispatch(SpouseAction::SetField { name, value });
            dirty.set(true);
        })
    };

    // ── Bool change handler ───────────────────────────────────────────────────
    let handle_bool_change = |field: &'static str| {
        let spouse = spouse.dispatcher();
        let dirty = dirty.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            spouse.dispatch(SpouseAction::SetBool {
                name: field.to_string(),
                value: input.checked(),
            });
            dirty.set(true);
        })
    };

    // ── Save ──────────────────────────────────────────────────────────────────
    let on_save = {
        let spouse = spouse.clone();
        let saving = saving.clone();
        let dirty = dirty.clone();
        let error = error.clone();
        let is_existing = is_existing.clone();
        Callback::from(move |_: MouseEvent| {
            let payload = (*spouse).clone();
            let saving = saving.clone();
            let dirty = dirty.clone();
            let error = error.clone();
            let is_existing = is_existing.clone();
            saving.set(true);
            spawn_local(async move {
                match save_spouse(payload).await {
                    Ok(_) => {
                        dirty.set(false);
                        error.set(None);
                        is_existing.set(true);
                    }
                    Err(e) => error.set(Some(e)),
                }
                saving.set(false);
            });
        })
    };

    let has_family = props.selected_family_id.is_some();

    html! {
        <aside class="member-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                        <circle cx="12" cy="7" r="4"/>
                    </svg>
                </span>
                <h3 class="works-form-title">{ "Spouse Information" }</h3>

                if let Some(fid) = &props.selected_family_id {
                    <span class="works-table-count" style="margin-left: auto">
                        { format!("Family {}", fid) }
                    </span>
                    if *is_existing {
                        <span class="family-badge family-badge--yes">{ "Saved" }</span>
                    } else {
                        <span class="family-badge family-badge--no">{ "New" }</span>
                    }
                }
            </div>

            if !has_family {
                <div class="works-table-empty" style="padding: var(--space-8)">
                    <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
                        class="works-table-empty-icon">
                        <circle cx="12" cy="8" r="4"/>
                        <path d="M4 21v-2a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v2"/>
                    </svg>
                    <span class="works-table-empty-text">{ "No family selected" }</span>
                    <span class="works-table-empty-sub">
                        { "Double-click a family record to load spouse data" }
                    </span>
                </div>

            } else if *loading {
                <div class="works-table-empty" style="padding: var(--space-8)">
                    <span class="works-table-empty-text">{ "Loading..." }</span>
                </div>

            } else {
                if let Some(err) = (*error).as_deref() {
                    <div class="works-form-error member-form-full">
                        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="10"/>
                            <line x1="12" y1="8" x2="12" y2="12"/>
                            <line x1="12" y1="16" x2="12.01" y2="16"/>
                        </svg>
                        { err }
                    </div>
                }

                <div class="member-form-body">
                    <div class="member-form-section-label member-form-full">{ "Identity" }</div>

                    <div class="form member-form-field">
                        <input type="text" name="first_name" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ spouse.first_name.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="first_name" class="form-label">{ "First Name" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="last_name" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ spouse.last_name.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="last_name" class="form-label">{ "Last Name" }</label>
                    </div>

                    <div class="form member-form-field member-form-full">
                        <input type="text" name="date_of_birth" class="form-input"
                            placeholder="YYYY-MM-DD" pattern=r"\d{4}-\d{2}-\d{2}"
                            value={ spouse.date_of_birth.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="date_of_birth" class="form-label">{ "Date of Birth" }</label>
                    </div>

                    <div class="member-form-section-label member-form-full">{ "Status" }</div>

                    <div class="form-checkbox member-form-field">
                        <input type="checkbox" id="spouse_is_member" name="is_member"
                            class="form-checkbox-input" checked={ spouse.is_member }
                            onchange={ handle_bool_change("is_member") } />
                        <label for="spouse_is_member" class="form-checkbox-label">{ "Member?" }</label>
                    </div>

                    <div class="form-checkbox member-form-field">
                        <input type="checkbox" id="spouse_is_active" name="is_active"
                            class="form-checkbox-input" checked={ spouse.is_active }
                            onchange={ handle_bool_change("is_active") } />
                        <label for="spouse_is_active" class="form-checkbox-label">{ "Active?" }</label>
                    </div>

                    <div class="form-checkbox member-form-field member-form-full">
                        <input type="checkbox" id="spouse_bulletin" name="on_bulletin_email_list"
                            class="form-checkbox-input" checked={ spouse.on_bulletin_email_list }
                            onchange={ handle_bool_change("on_bulletin_email_list") } />
                        <label for="spouse_bulletin" class="form-checkbox-label">
                            { "Bulletin Email List?" }
                        </label>
                    </div>

                    <div class="member-form-section-label member-form-full">{ "Contact" }</div>

                    <div class="form member-form-field">
                        <input type="text" name="cell_phone" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ spouse.cell_phone.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="cell_phone" class="form-label">{ "Cell Phone" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="work_phone" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ spouse.work_phone.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="work_phone" class="form-label">{ "Work Phone" }</label>
                    </div>

                    <div class="form member-form-field member-form-full">
                        <input type="text" name="email_address" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ spouse.email_address.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="email_address" class="form-label">{ "Email Address" }</label>
                    </div>
                </div>

                <div class="works-form-footer">
                    <button
                        class="btn btn-primary works-form-submit"
                        onclick={ on_save }
                        disabled={ !*dirty || *saving }
                    >
                        { if *saving { "Saving..." } else if *is_existing { "Save Changes" } else { "Add Spouse" } }
                    </button>
                </div>
            }
        </aside>
    }
}
