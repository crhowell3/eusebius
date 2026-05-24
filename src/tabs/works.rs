use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone)]
struct Work {
    work_code: String,
    description: String,
}

#[function_component(Works)]
pub fn works() -> Html {
    //  TODO(@cameron): actually use form_error
    let _form_error = use_state(|| "".to_string());
    let new_work = use_state(|| Work {
        work_code: "".to_string(),
        description: "".to_string(),
    });

    let handle_work_change = {
        let new_work = new_work.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            let (name, value) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                (input.name(), input.value())
            } else if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
                (textarea.name(), textarea.value())
            } else {
                return;
            };

            let mut updated = (*new_work).clone();

            match name.as_str() {
                "work_code" => updated.work_code = value,
                "description" => updated.description = value,
                _ => {
                    unreachable!()
                }
            }
            new_work.set(updated);
        })
    };

    //  TODO(@cameron): implement this
    let add_work = { Callback::from(move |_| unimplemented!()) };

    html! {
        <div>
            // Members Table View
            <table>
                <thead>
                    <tr>
                        <th>
                            <input type="checkbox"/>
                        </th>
                    </tr>
                </thead>
                <tbody>

                </tbody>
            </table>

            <h3 style="margin-bottom: 1rem">{"Add Work"}</h3>
            <div class="form">
                <input
                    type="text"
                    name="work_code"
                    autoComplete="off"
                    class="form-input"
                    placeholder=""
                    minLength=2
                    maxLength=2
                    value={(*new_work).clone().work_code}
                    onchange={handle_work_change.clone()}
                />
                <label htmlFor="work_code" class="form-label">
                    {"Work Code"}
                </label>
            </div>

            <div class="form">
                <input
                    type="text"
                    name="description"
                    class="form-input"
                    placeholder=""
                    value={(*new_work).clone().description}
                    onchange={handle_work_change.clone()}
                />
                <label htmlFor="description" class="form-label">
                    {"Description"}
                </label>
            </div>

            <button
                class="btn btn-primary"
                onclick={add_work}
                disabled={new_work.work_code.is_empty() || new_work.description.is_empty()}
            >
                {"Add Work"}
            </button>
        </div>
    }
}
