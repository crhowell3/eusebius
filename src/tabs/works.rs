use yew::prelude::*;

use crate::components::forms::WorksForm;
use crate::components::tables::WorksTable;

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
