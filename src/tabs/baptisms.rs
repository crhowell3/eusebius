use yew::prelude::*;

use crate::components::forms::BaptismsForm;
use crate::components::tables::BaptismsTable;

#[function_component(BaptismsTabBody)]
pub fn baptisms_tab_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);

    let on_baptism_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    html! {
        <div class="works-layout">
            <BaptismsForm on_baptism_added={ on_baptism_added } />
            <BaptismsTable refresh_trigger={ *refresh_trigger } />
        </div>
    }
}
