use yew::prelude::*;

pub mod children;
pub mod families;
pub mod spouses;

use children::Children;
use families::FamilySectionBody;
use spouses::Spouses;

#[function_component(MemberTabBody)]
pub fn member_tab_body() -> Html {
    html! {
        <>
            <FamilySectionBody />
            <Spouses />
            <Children />
        </>
    }
}
