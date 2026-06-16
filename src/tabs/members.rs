use yew::prelude::*;

pub mod children;
pub mod families;
pub mod spouses;

use children::Children;
use families::FamilySectionBody;
use spouses::Spouses;

#[function_component(MemberTabBody)]
pub fn member_tab_body() -> Html {
    let selected_family_id = use_state(|| None::<String>);

    html! {
        <div class="member-tab-root">
            <FamilySectionBody selected_family_id={ selected_family_id.clone() } />
            <div class="member-bottom-section">
                <div class="member-spouse-pane">
                    <Spouses />
                </div>
                <div class="member-children-pane">
                    <Children selected_family_id={ (*selected_family_id).clone() } />
                </div>
            </div>
        </div>
    }
}
