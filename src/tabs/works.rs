use yew::prelude::*;

#[derive(Clone)]
struct Work {
    work_code: String,
    description: String,
}

#[function_component(Works)]
pub fn works() -> Html {
    let form_error = use_state(|| "".to_string());
    let new_work = use_state(|| Work {
        work_code: "".to_string(),
        description: "".to_string(),
    });

    html! {
        <div>
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
                />
                <label htmlFor="description" class="form-label">
                    {"Description"}
                </label>
            </div>

            <button class="btn btn-primary" disabled=false>{"Add Work"}</button>
        </div>
    }
}
