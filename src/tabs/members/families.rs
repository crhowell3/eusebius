use yew::prelude::*;

pub mod families_form;
pub mod families_table;

use families_form::FamiliesForm;
use families_table::FamiliesTable;

#[function_component(FamilySectionBody)]
pub fn family_section_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);

    let on_family_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    html! {
        <div class="works-layout">
            <FamiliesForm on_family_added={ on_family_added }/>
            <FamiliesTable refresh_trigger={ *refresh_trigger }/>
        </div>
    }
}
