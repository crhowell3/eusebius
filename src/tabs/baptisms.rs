use yew::prelude::*;

pub mod baptisms_form;
pub mod baptisms_table;

use baptisms_form::BaptismsForm;
use baptisms_table::BaptismsTable;

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
