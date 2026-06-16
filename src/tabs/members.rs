use yew::prelude::*;

pub mod children;
pub mod families;
pub mod spouses;

use crate::components::forms::FamiliesForm;
use children::Children;
use families::FamilySectionBody;
use spouses::Spouses;

#[function_component(MemberTabBody)]
pub fn member_tab_body() -> Html {
    let refresh_trigger = use_state(|| 0u32);
    let selected_family_id = use_state(|| None::<String>);

    let form_open = use_state(|| false);

    let on_toggle_form = {
        let form_open = form_open.clone();
        Callback::from(move |_: MouseEvent| form_open.set(!*form_open))
    };

    let on_close_form = {
        let form_open = form_open.clone();
        Callback::from(move |_: MouseEvent| form_open.set(false))
    };

    // Clicking the backdrop closes the panel
    let on_backdrop_click = {
        let form_open = form_open.clone();
        Callback::from(move |_: MouseEvent| form_open.set(false))
    };

    html! {
        <div class="member-tab-root">
            <div class="member-tab-toolbar">
                <button class="member-form-toggle" onclick={ on_toggle_form }>
                    if *form_open {
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="18" y1="6" x2="6" y2="18"/>
                            <line x1="6" y1="6" x2="18" y2="18"/>
                        </svg>
                        { " Close Form" }
                    } else {
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="12" y1="5" x2="12" y2="19" />
                            <line x1="5" y1="12" x2="19" y2="12" />
                        </svg>
                        { " Add Member" }
                    }
                </button>
            </div>

            if *form_open {
                <div class="slide-panel-backdrop" onclick={ on_backdrop_click } />
            }

            <div class={ if *form_open { "slide-panel slide-panel--open" } else { "slide-panel" } }>
                <div class="slide-panel-header">
                    <h3 class="slide-panel-title">{ "New Member" }</h3>
                    <button class="slide-panel-close" onclick={ on_close_form }>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                            viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="18" y1="6" x2="6" y2="18"/>
                            <line x1="6" y1="6" x2="18" y2="18"/>
                        </svg>
                    </button>
                </div>
                <div class="slide-panel-body">
                    <FamiliesForm on_family_added={
                        {
                            let form_open = form_open.clone();
                            let refresh_trigger = refresh_trigger.clone();
                            Callback::from(move |_: ()| {
                                refresh_trigger.set(*refresh_trigger + 1); form_open.set(false)
                            })
                        }
                    } />
                </div>
            </div>
            <div class="member-tab-content">
                <FamilySectionBody refresh_trigger={ *refresh_trigger } selected_family_id={ selected_family_id.clone() } />
                <div class="member-bottom-section">
                    <div class="member-spouse-pane">
                        <Spouses />
                    </div>
                    <div class="member-children-pane">
                        <Children selected_family_id={ (*selected_family_id).clone() } />
                    </div>
                </div>
            </div>
        </div>
    }
}
