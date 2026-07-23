use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::invoke;
use models::{Category, CategoryAction};

async fn add_category(name: String) -> Result<Category, String> {
    #[derive(Serialize)]
    struct Args {
        name: String,
    }
    let args = to_value(&Args { name }).map_err(|e| e.to_string())?;
    let result = invoke("add_category", args).await;
    from_value::<Category>(result).map_err(|e| e.to_string())
}

#[derive(Properties, PartialEq)]
pub struct CategoriesFormProps {
    pub on_category_added: Callback<()>,
}

#[function_component(CategoriesForm)]
pub fn categories_form(props: &CategoriesFormProps) -> Html {
    let form_error = use_state(|| None::<String>);
    let new_category = use_reducer(Category::default);

    let on_name_input = {
        let new_category = new_category.dispatcher();
        Callback::from(move |e: InputEvent| {
            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                new_category.dispatch(CategoryAction::SetName(input.value()));
            }
        })
    };

    let handle_submit = {
        let new_category = new_category.clone();
        let form_error = form_error.clone();
        let on_category_added = props.on_category_added.clone();
        Callback::from(move |_: MouseEvent| {
            let name = new_category.name.clone();
            let form_error = form_error.clone();
            let new_category = new_category.dispatcher();
            let on_category_added = on_category_added.clone();
            spawn_local(async move {
                match add_category(name).await {
                    Ok(_) => {
                        new_category.dispatch(CategoryAction::Reset);
                        form_error.set(None);
                        on_category_added.emit(());
                    }
                    Err(e) => form_error.set(Some(e)),
                }
            });
        })
    };

    let is_valid = !new_category.name.trim().is_empty();

    html! {
        <aside class="works-form-card">
            <div class="works-form-header">
                <span class="works-form-header-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                        fill="none" stroke="currentColor" stroke-width="2.5"
                        stroke-linecap="round" stroke-linejoin="round">
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                </span>
                <h3 class="works-form-title">{ "New Category" }</h3>
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

                <div class="works-form-hint">{ "Tags are assigned automatically" }</div>

                <div class="form">
                    <input
                        type="text"
                        name="name"
                        autocomplete="off"
                        class="form-input"
                        placeholder=""
                        value={ new_category.name.clone() }
                        oninput={ on_name_input }
                    />
                    <label for="name" class="form-label">
                        { "Category Name" }
                    </label>
                </div>
            </div>

            <div class="works-form-footer">
                <button
                    class="btn btn-primary works-form-submit"
                    onclick={ handle_submit }
                    disabled={ !is_valid }
                >
                    { "Add Category" }
                </button>
            </div>
        </aside>
    }
}
