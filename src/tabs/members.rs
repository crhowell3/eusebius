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
        <>
            <FamilySectionBody selected_family_id={ selected_family_id.clone() } />
            <Spouses />
            <Children selected_family_id={ (*selected_family_id).clone() } />
        </>
    }
}
