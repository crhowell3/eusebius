use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;

use shared::Family;

#[derive(Properties, PartialEq)]
pub struct FamiliesTableProps {
    pub refresh_trigger: u32,
}

async fn fetch_families() -> Result<Vec<Family>, String> {
    let result = invoke("get_families", JsValue::UNDEFINED).await;
    from_value::<Vec<Family>>(result).map_err(|e| e.to_string())
}

#[function_component(FamiliesTable)]
pub fn families_table(props: &FamiliesTableProps) -> Html {
    let members = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let members = members.clone();
        let loading = loading.clone();
        let error = error.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            loading.set(true);
            spawn_local(async move {
                match fetch_families().await {
                    Ok(data) => {
                        members.set(data);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="table-panel">
            <h3 class="panel-title">{"Family Records"}</h3>

            <button
                class="btn btn-icon danger"
                disabled=false
                //onClick={() => setShowConfirm(true)}
                title="Delete selected"
                aria-label="Delete selected"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="18"
                    height="18"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                >
                    <polyline points="3 6 5 6 21 6" />
                    <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
                    <path d="M10 11v6" />
                    <path d="M14 11v6" />
                    <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" />
                </svg>
            </button>

            if let Some(err) = (*error).clone() {
                <p class="text-error">{ err }</p>
            }

            if *loading {
                <p class="text-muted">{ "Loading..." }</p>
            } else if (*members).is_empty() {
                <p class="text-muted">{ "No records found." }</p>
            } else {
                <table class="results-table">
                    <thead>
                        <tr>
                            <th>{ "Family ID" }</th>
                            <th>{ "Mail Route "}</th>
                            <th>{ "Last Name" }</th>
                            <th>{ "First Name" }</th>
                            <th>{ "Member?" }</th>
                            <th>{ "Active?" }</th>
                            <th>{ "DOB" }</th>
                            <th>{ "Anniversary Month" }</th>
                            <th>{ "Anniversary Day" }</th>
                            <th>{ "Home Phone" }</th>
                            <th>{ "Cell Phone" }</th>
                            <th>{ "Work Phone" }</th>
                            <th>{ "Address" }</th>
                            <th>{ "City" }</th>
                            <th>{ "State" }</th>
                            <th>{ "ZIP" }</th>
                            <th>{ "E-Mail Address" }</th>
                            <th>{ "Bulletin E-Mail List?" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*members).iter().map(|m| html! {
                            <tr key={m.family_id.clone()}>
                                <td>{ &m.family_id }</td>
                                <td>{ &m.mail_route }</td>
                                <td>{ &m.last_name }</td>
                                <td>{ &m.first_name }</td>
                                <td>{ &m.is_member }</td>
                                <td>{ &m.is_active }</td>
                                <td>{ &m.date_of_birth }</td>
                                <td>{ &m.anniversary_month }</td>
                                <td>{ &m.anniversary_day }</td>
                                <td>{ &m.home_phone }</td>
                                <td>{ &m.cell_phone }</td>
                                <td>{ &m.work_phone }</td>
                                <td>{ &m.address }</td>
                                <td>{ &m.city }</td>
                                <td>{ &m.state }</td>
                                <td>{ &m.zip }</td>
                                <td>{ &m.email_address }</td>
                                <td>{ &m.on_bulletin_email_list }</td>
                            </tr>
                        })}
                    </tbody>
                </table>
            }
        </div>
    }
}
