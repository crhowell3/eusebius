use yew::prelude::*;
pub mod families_table;

use families_table::FamiliesTable;

#[derive(Properties, PartialEq)]
pub struct FamilySectionProps {
    pub selected_family_id: UseStateHandle<Option<String>>,
}

#[function_component(FamilySectionBody)]
pub fn family_section_body(props: &FamilySectionProps) -> Html {
    let refresh_trigger = use_state(|| 0u32);
    let selected_family_id = props.selected_family_id.clone();
    let on_select_family = {
        let selected_family_id = selected_family_id.clone();
        Callback::from(move |family_id: String| {
            selected_family_id.set(Some(family_id));
        })
    };

    html! {
        <div class="member-layout">
            <div class="member-table-pane">
                <FamiliesTable
                    refresh_trigger={ *refresh_trigger }
                    on_select_family={ on_select_family }
                    selected_family_id={ (*selected_family_id).clone() }
                />
            </div>
        </div>
    }
}
