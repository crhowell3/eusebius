use yew::prelude::*;

use crate::components::forms::DeathsForm;
use crate::components::tables::DeathsTable;

#[function_component(DeathTabBody)]
pub fn death_tab_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);

    let on_death_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    html! {
        <div class="works-layout">
            <DeathsForm on_death_added={ on_death_added }/>
            <DeathsTable refresh_trigger={ *refresh_trigger } />
        </div>
    }
}
