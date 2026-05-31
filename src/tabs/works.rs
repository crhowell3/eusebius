use yew::prelude::*;

pub mod works_form;
pub mod works_table;

use works_form::WorksForm;
use works_table::WorksTable;

#[function_component(WorkTabBody)]
pub fn work_tab_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);

    let on_work_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    html! {
        <div class="works-layout">
            <WorksForm on_work_added={ on_work_added }/>
            <WorksTable refresh_trigger={ *refresh_trigger } />
        </div>
    }
}
