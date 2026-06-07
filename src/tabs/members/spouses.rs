use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::utils::invoke;
use shared::{GenericAction, Spouse};

#[function_component(Spouses)]
pub fn spouses() -> Html {
    let form_error = use_state(|| None::<String>);
    let spouse = use_reducer(Spouse::default);

    let handle_spouse_change = {
        let spouse = spouse.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            spouse.dispatch(GenericAction::SetField { name, value });
        })
    };

    html! {
        <div class="member-layout">
            <aside class="member-form-card">
                <div class="works-form-header">
                    <h3 class="works-form-title">{ "Spouse Information" }</h3>
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

                    <div class="form member-form-field">
                        <input type="text" name="date_of_birth" class="form-input"
                            placeholder="YYYY-MM-DD" pattern=r"\d{4}-\d{2}-\d{2}"
                            value={ spouse.date_of_birth.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="date_of_birth" class="form-label">{ "Date of Birth" }</label>
                    </div>

                    <div class="member-form-section-label member-form-full">{ "Status" }</div>

                    <div class="form-checkbox member-form-field">
                        <input type="checkbox" id="is_member" name="is_member"
                            class="form-checkbox-input"
                            checked={ spouse.is_member }
                            onchange={{
                                let spouse = spouse.dispatcher();
                                Callback::from(move |e: Event| {
                                    let input = e.target().unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
                                    spouse.dispatch(GenericAction::SetBool {
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
                            checked={ spouse.is_active }
                            onchange={{
                                let spouse = spouse.dispatcher();
                                Callback::from(move |e: Event| {
                                    let input = e.target().unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
                                    spouse.dispatch(GenericAction::SetBool {
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
                            checked={ spouse.on_bulletin_email_list }
                            onchange={{
                                let spouse = spouse.dispatcher();
                                Callback::from(move |e: Event| {
                                    let input = e.target().unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
                                    spouse.dispatch(GenericAction::SetBool {
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

                    <div class="member-form-field" />

                    <div class="form member-form-field member-form-full">
                        <input type="text" name="email_address" class="form-input"
                            placeholder=" " autocomplete="off"
                            value={ spouse.email_address.clone() }
                            oninput={ handle_spouse_change.clone() } />
                        <label for="email_address" class="form-label">{ "Email Address" }</label>
                    </div>
                </div>

            </aside>
        </div>
    }
}
