use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::utils::invoke;
use models::{Family, GenericAction};

#[derive(Serialize)]
struct AddFamilyArgs {
    family: Family,
}

async fn add_family(family: Family) -> Result<(), String> {
    let args = to_value(&AddFamilyArgs { family }).map_err(|e| e.to_string())?;
    let _ = invoke("add_family", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

fn is_valid_mail_route(s: &str) -> bool {
    s.len() == 3 && s.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_family(f: &Family) -> bool {
    is_valid_mail_route(&f.mail_route)
        && !f.first_name.trim().is_empty()
        && !f.last_name.trim().is_empty()
}

#[derive(Properties, PartialEq)]
pub struct FamiliesFormProps {
    pub on_family_added: Callback<()>,
}

#[function_component(FamiliesForm)]
pub fn families_form(props: &FamiliesFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_family = use_reducer(Family::default);

    let handle_family_change = {
        let new_family = new_family.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                (select.name(), select.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            new_family.dispatch(GenericAction::SetField { name, value });
        })
    };

    let handle_submit = {
        let new_family = new_family.clone();
        let form_error = form_error.clone();
        let on_family_added = props.on_family_added.clone();
        Callback::from(move |_: MouseEvent| {
            let family = (*new_family).clone();
            let form_error = form_error.clone();
            let new_family = new_family.dispatcher();
            let on_family_added = on_family_added.clone();
            spawn_local(async move {
                match add_family(family).await {
                    Ok(_) => {
                        new_family.dispatch(GenericAction::Reset);
                        form_error.set(None);
                        on_family_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            });
        })
    };

    let is_valid = is_valid_family(&*new_family);

    html! {
        <div>
            <aside class="member-form-card">

                <div class="works-form-header">
                    <span class="works-form-header-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                            fill="none" stroke="currentColor" stroke-width="2.5"
                            stroke-linecap="round" stroke-linejoin="round">
                            <line x1="12" y1="5" x2="12" y2="19" />
                            <line x1="5" y1="12" x2="19" y2="12" />
                        </svg>
                    </span>
                    <h3 class="works-form-title">{ "New Member Information" }</h3>
                </div>

                <div class="member-form-body">
                    if let Some(err) = (*form_error).as_deref() {
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

                    <div class="member-form-section-label member-form-full">{ "Identity" }</div>

                    <div class="form member-form-field">
                        <input type="text" name="first_name" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ new_family.first_name.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="first_name" class="form-label">{ "First Name" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="last_name" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ new_family.last_name.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="last_name" class="form-label">{ "Last Name" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="mail_route" class="form-input"
                            placeholder="123" pattern=r"\d{3}" autocomplete="off"
                            minlength="3" maxlength="3" inputmode="numeric"
                            value={ new_family.mail_route.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="mail_route" class="form-label">{ "Mail Route" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="date_of_birth" class="form-input"
                            placeholder="YYYY-MM-DD" pattern=r"\d{4}-\d{2}-\d{2}"
                            minlength="10" maxlength="10"
                            value={ new_family.date_of_birth.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="date_of_birth" class="form-label">{ "Date of Birth" }</label>
                    </div>

                    <div class="member-form-section-label member-form-full">{ "Status" }</div>

                    <div class="form-checkbox member-form-field">
                        <input type="checkbox" id="is_member" name="is_member"
                            class="form-checkbox-input"
                            checked={ new_family.is_member }
                            onchange={{
                                let new_family = new_family.dispatcher();
                                Callback::from(move |e: Event| {
                                    let input = e.target().unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
                                    new_family.dispatch(GenericAction::SetBool {
                                        name: "is_member".to_string(),
                                        value: input.checked(),
                                    });
                                })
                            }}
                        />
                        <label for="is_member" class="form-checkbox-label">{ "Member?" }</label>
                    </div>

                    <div class="form-checkbox member-form-field">
                        <input type="checkbox" id="is_active" name="is_active"
                            class="form-checkbox-input"
                            checked={ new_family.is_active }
                            onchange={{
                                let new_family = new_family.dispatcher();
                                Callback::from(move |e: Event| {
                                    let input = e.target().unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
                                    new_family.dispatch(GenericAction::SetBool {
                                        name: "is_active".to_string(),
                                        value: input.checked(),
                                    });
                                })
                            }}
                        />
                        <label for="is_active" class="form-checkbox-label">{ "Active?" }</label>
                    </div>

                    <div class="form-checkbox member-form-field">
                        <input type="checkbox" id="on_bulletin_email_list"
                            name="on_bulletin_email_list" class="form-checkbox-input"
                            checked={ new_family.on_bulletin_email_list }
                            onchange={{
                                let new_family = new_family.dispatcher();
                                Callback::from(move |e: Event| {
                                    let input = e.target().unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
                                    new_family.dispatch(GenericAction::SetBool {
                                        name: "on_bulletin_email_list".to_string(),
                                        value: input.checked(),
                                    });
                                })
                            }}
                        />
                        <label for="on_bulletin_email_list" class="form-checkbox-label">
                            { "Bulletin Email List?" }
                        </label>
                    </div>

                    <div class="member-form-section-label member-form-full">{ "Anniversary" }</div>

                    <div class="form member-form-field">
                        <select name="anniversary_month" class="form-input form-select"
                            oninput={ handle_family_change.clone() }>
                            <option value="" disabled=true
                                selected={ new_family.anniversary_month.is_empty() }>
                                { "" }
                            </option>
                            { for [
                                ("Jan", "January"), ("Feb", "February"), ("Mar", "March"),
                                ("Apr", "April"),   ("May", "May"),      ("Jun", "June"),
                                ("Jul", "July"),    ("Aug", "August"),   ("Sep", "September"),
                                ("Oct", "October"), ("Nov", "November"), ("Dec", "December"),
                            ].iter().map(|(val, label)| {
                                let selected = new_family.anniversary_month == *val;
                                html! { <option value={*val} selected={selected}>{ label }</option> }
                            })}
                        </select>
                        <label for="anniversary_month" class="form-label">{ "Month" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="anniversary_day" autocomplete="off"
                            class="form-input" placeholder="DD"
                            minlength="2" maxlength="2"
                            pattern=r"\d{2}"
                            value={ new_family.anniversary_day.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="anniversary_day" class="form-label">{ "Day" }</label>
                    </div>

                    <div class="member-form-field" />

                    <div class="member-form-section-label member-form-full">{ "Contact" }</div>

                    <div class="form member-form-field">
                        <input type="text" name="home_phone" autocomplete="off"
                            class="form-input"
                            placeholder="(999) 999-9999"
                            maxlength="14"
                            value={ new_family.home_phone.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="home_phone" class="form-label">{ "Home Phone" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="cell_phone" autocomplete="off"
                            class="form-input"
                            placeholder="(999) 999-9999"
                            maxlength="14"
                            value={ new_family.cell_phone.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="cell_phone" class="form-label">{ "Cell Phone" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="work_phone" autocomplete="off"
                            class="form-input"
                            placeholder="(999) 999-9999"
                            maxlength="14"
                            value={ new_family.work_phone.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="work_phone" class="form-label">{ "Work Phone" }</label>
                    </div>

                    <div class="form member-form-field member-form-full">
                        <input type="text" name="email_address" autocomplete="off"
                            class="form-input"
                            placeholder="user@gmail.com"
                            pattern=r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$"
                            value={ new_family.email_address.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="email_address" class="form-label">{ "Email Address" }</label>
                    </div>

                    <div class="member-form-section-label member-form-full">{ "Address" }</div>

                    <div class="form member-form-field member-form-full">
                        <input type="text" name="address" autocomplete="off"
                            class="form-input" placeholder=" "
                            value={ new_family.address.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="address" class="form-label">{ "Street Address" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="city" autocomplete="off"
                            class="form-input" placeholder=" "
                            value={ new_family.city.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="city" class="form-label">{ "City" }</label>
                    </div>

                    <div class="form member-form-field">
                        <select name="state" class="form-input form-select"
                            oninput={ handle_family_change.clone() }>
                            <option value="" disabled=true
                                selected={ new_family.state.is_empty() }>
                                { "" }
                            </option>
                            { for [
                                ("AL", "Alabama"),
                                ("AK", "Alaska"),
                                ("AZ", "Arizona"),
                                ("AR", "Arkansas"),
                                ("CA", "California"),
                                ("CO", "Colorado"),
                                ("CT", "Connecticut"),
                                ("DE", "Delaware"),
                                ("DC", "District of Columbia"),
                                ("FL", "Florida"),
                                ("GA", "Georgia"),
                                ("HI", "Hawaii"),
                                ("ID", "Idaho"),
                                ("IL", "Illinois"),
                                ("IN", "Indiana"),
                                ("IA", "Iowa"),
                                ("KS", "Kansas"),
                                ("KY", "Kentucky"),
                                ("LA", "Louisiana"),
                                ("ME", "Maine"),
                                ("MD", "Maryland"),
                                ("MA", "Massachusetts"),
                                ("MI", "Michigan"),
                                ("MN", "Minnesota"),
                                ("MS", "Mississippi"),
                                ("MO", "Missouri"),
                                ("MT", "Montana"),
                                ("NE", "Nebraska"),
                                ("NV", "Nevada"),
                                ("NH", "New Hampshire"),
                                ("NJ", "New Jersey"),
                                ("NM", "New Mexico"),
                                ("NY", "New York"),
                                ("NC", "North Carolina"),
                                ("ND", "North Dakota"),
                                ("OH", "Ohio"),
                                ("OK", "Oklahoma"),
                                ("OR", "Oregon"),
                                ("PA", "Pennsylvania"),
                                ("RI", "Rhode Island"),
                                ("SC", "South Carolina"),
                                ("SD", "South Dakota"),
                                ("TN", "Tennessee"),
                                ("TX", "Texas"),
                                ("UT", "Utah"),
                                ("VT", "Vermont"),
                                ("VA", "Virginia"),
                                ("WA", "Washington"),
                                ("WV", "West Virginia"),
                                ("WI", "Wisconsin"),
                                ("WY", "Wyoming"),
                            ].iter().map(|(val, label)| {
                                let selected = new_family.state == *val;
                                html! { <option value={*val} selected={selected}>{ label }</option> }
                            })}
                        </select>
                        <label for="state" class="form-label">{ "State" }</label>
                    </div>

                    <div class="form member-form-field">
                        <input type="text" name="zip" autocomplete="off"
                            class="form-input"
                            placeholder="00000"
                            minlength="5" maxlength="5" inputmode="numeric"
                            pattern=r"\d{5}"
                            value={ new_family.zip.clone() }
                            oninput={ handle_family_change.clone() } />
                        <label for="zip" class="form-label">{ "ZIP" }</label>
                    </div>

                </div>

                <div class="works-form-footer">
                    <button
                        class="btn btn-primary works-form-submit"
                        onclick={ handle_submit }
                        disabled={ !is_valid }
                    >
                        { "Add Member" }
                    </button>
                </div>

            </aside>
        </div>
    }
}
