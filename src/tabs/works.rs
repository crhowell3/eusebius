use yew::prelude::*;

use crate::components::forms::{CategoriesForm, WorksForm};
use crate::components::tables::{CategoriesTable, WorksTable};

#[function_component(WorkTabBody)]
pub fn work_tab_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);

    let on_work_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    let on_category_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    let on_category_delete = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    html! {
        <div>
            <div class="works-layout">
                <CategoriesForm on_category_added={ on_category_added }/>
                <CategoriesTable on_category_delete={ on_category_delete } refresh_trigger={ *refresh_trigger } />
            </div>
            <div class="works-layout">
                <WorksForm refresh_trigger={ *refresh_trigger } on_work_added={ on_work_added }/>
                <WorksTable refresh_trigger={ *refresh_trigger } />
            </div>
        </div>
    }
}
