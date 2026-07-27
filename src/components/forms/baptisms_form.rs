use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::components::icons::Plus;
use crate::utils::invoke;
use models::{Baptism, GenericAction};

#[derive(Serialize)]
struct AddBaptismArgs {
    baptism: Baptism,
}

async fn add_baptism(baptism: Baptism) -> Result<(), String> {
    let args = to_value(&AddBaptismArgs { baptism }).map_err(|e| e.to_string())?;
    let _ = invoke("add_baptism", args)
        .await
        .map_err(|e| e.as_string().unwrap_or("Unknown error".to_string()))?;

    Ok(())
}

fn is_valid_family_id(s: &str) -> bool {
    s.len() == 4 && s.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_date(s: &str) -> bool {
    if s.len() != 10 {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    let Ok(y) = parts[0].parse::<u32>() else {
        return false;
    };
    let Ok(m) = parts[1].parse::<u32>() else {
        return false;
    };
    let Ok(d) = parts[2].parse::<u32>() else {
        return false;
    };
    y >= 1000 && (1..=12).contains(&m) && (1..=31).contains(&d)
}

fn is_valid_baptism(b: &Baptism) -> bool {
    is_valid_family_id(&b.family_id)
        && !b.first_name.trim().is_empty()
        && !b.last_name.trim().is_empty()
        && is_valid_date(&b.date_baptized)
        && !b.witness.trim().is_empty()
        && !b.location.trim().is_empty()
}

#[derive(Properties, PartialEq)]
pub struct BaptismsFormProps {
    pub on_baptism_added: Callback<()>,
}

#[function_component(BaptismsForm)]
pub fn baptisms_form(props: &BaptismsFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_baptism = use_reducer(Baptism::default);

    let handle_baptism_change = {
        let new_baptism = new_baptism.dispatcher();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            new_baptism.dispatch(GenericAction::SetField { name, value });
        })
    };

    let handle_submit = {
        let new_baptism = new_baptism.clone();
        let form_error = form_error.clone();
        let on_baptism_added = props.on_baptism_added.clone();
        Callback::from(move |_: MouseEvent| {
            let baptism = (*new_baptism).clone();
            let form_error = form_error.clone();
            let new_baptism = new_baptism.dispatcher();
            let on_baptism_added = on_baptism_added.clone();
            spawn_local(async move {
                match add_baptism(baptism).await {
                    Ok(_) => {
                        new_baptism.dispatch(GenericAction::Reset);
                        form_error.set(None);
                        on_baptism_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            })
        })
    };

    let is_valid = is_valid_baptism(&*new_baptism);

    html! {
        <aside class="works-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    <Plus />
                </span>
                <h3 class="works-form-title">{ "Add Baptism" }</h3>
            </div>

            <div class="works-form-body">
                if let Some(err) = (*form_error).as_deref() {
                    <div class="works-form-error">
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

                <div class="form">
                    <input
                        type="text"
                        name="family_id"
                        class="form-input"
                        placeholder="4-digit number"
                        pattern=r"\d{4}"
                        autoComplete="off"
                        minlength="4"
                        maxlength="4"
                        inputmode="numeric"
                        value={ new_baptism.family_id.clone() }
                        oninput={ handle_baptism_change.clone() }
                    />
                    <label htmlFor="family_id" class="form-label">
                        {"Family ID"}
                    </label>
                </div>

                <div class="form">
                    <input
                        type="text"
                        name="first_name"
                        autoComplete="off"
                        class="form-input"
                        placeholder=" "
                        value={ new_baptism.first_name.clone() }
                        oninput={ handle_baptism_change.clone() }
                    />
                    <label htmlFor="first_name" class="form-label">
                        {"First Name"}
                    </label>
                </div>

                <div class="form">
                    <input
                        type="text"
                        name="last_name"
                        autoComplete="off"
                        class="form-input"
                        placeholder=" "
                        value={ new_baptism.last_name.clone() }
                        oninput={ handle_baptism_change.clone() }
                    />
                    <label htmlFor="last_name" class="form-label">
                        {"Last Name"}
                    </label>
                </div>

                <div class="member-form-section-label member-form-full">{ "Details" }</div>

                <div class="form">
                    <input
                        type="text"
                        name="date_baptized"
                        class="form-input"
                        placeholder="YYYY-MM-DD"
                        pattern=r"\d{4}-\d{2}-\d{2}"
                        value={ new_baptism.date_baptized.clone() }
                        oninput={ handle_baptism_change.clone() }
                    />
                    <label htmlFor="date_baptized" class="form-label">
                        {"Date"}
                    </label>
                </div>

                <div class="form">
                    <input
                        type="text"
                        name="witness"
                        class="form-input"
                        placeholder=" "
                        value={ new_baptism.witness.clone()}
                        oninput={ handle_baptism_change.clone() }
                    />
                    <label htmlFor="witness" class="form-label">
                        {"Witness"}
                    </label>
                </div>

                <div class="form">
                    <input
                        type="text"
                        name="location"
                        class="form-input"
                        placeholder=" "
                        value={ new_baptism.location.clone() }
                        oninput={ handle_baptism_change.clone() }
                    />
                    <label htmlFor="location" class="form-label">
                        {"Location"}
                    </label>
                </div>
            </div>

            <div class="works-form-footer">
                <button
                    class="btn btn-primary works-form-submit"
                    onclick={ handle_submit }
                    disabled={ !is_valid }
                >
                    { "Add Baptism" }
                </button>
            </div>
        </aside>
    }
}
