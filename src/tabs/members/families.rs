use yew::prelude::*;

pub mod families_form;
pub mod families_table;

use families_form::FamiliesForm;
use families_table::FamiliesTable;

#[function_component(FamilySectionBody)]
pub fn family_section_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);
    let form_collapsed = use_state(|| false);

    let on_family_added = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    let on_toggle_form = {
        let form_collapsed = form_collapsed.clone();
        Callback::from(move |_: MouseEvent| {
            form_collapsed.set(!*form_collapsed);
        })
    };

    html! {
        <div class="member-layout">
            <button
                class="member-form-toggle"
                onclick={ on_toggle_form }
                title={ if *form_collapsed { "Show form "} else { "Hide form" } }
            >
                if *form_collapsed {
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M9 18l6-6-6-6"/>
                    </svg>
                    { " Show Form" }
                } else {
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M15 18l-6-6 6-6"/>
                    </svg>
                    { " Hide Form" }
                }
            </button>
            <div class={ if *form_collapsed { "member-content member-content--collapsed" } else { "member-content" } }>
                if !*form_collapsed {
                    <div class="member-form-pane">
                        <FamiliesForm on_family_added={ on_family_added } />
                    </div>
                }
                <div class="member-table-pane">
                    <FamiliesTable refresh_trigger={ *refresh_trigger } />
                </div>
            </div>
        </div>
    }
}
