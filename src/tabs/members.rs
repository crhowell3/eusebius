use yew::prelude::*;

pub mod children;
pub mod families;
pub mod spouses;

use children::Children;
use families::Families;
use spouses::Spouses;

#[function_component(MemberTabBody)]
pub fn member_tab_body() -> Html {
    html! {
        <>
            <Families/>
            <Spouses/>
            <Children/>
        </>
    }
}
