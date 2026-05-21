use yew::prelude::*;

struct MenuButton {
    label: &'static str,
    id: &'static str,
    variant: &'static str,
}

const MENU_BUTTONS: &[MenuButton] = &[
    MenuButton {
        label: "Update Members",
        id: "update-members",
        variant: "primary",
    },
    MenuButton {
        label: "Update Works",
        id: "update-works",
        variant: "primary",
    },
    MenuButton {
        label: "Update Deaths",
        id: "update-deaths",
        variant: "primary",
    },
    MenuButton {
        label: "Update Baptisms",
        id: "update-baptisms",
        variant: "primary",
    },
];

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    html! {
        <div class="menu-backdrop">
            <div class="menu-panel">
                <div class="menu-header">
                    <h2 class="menu-title">{"Control Panel"}</h2>
                    <p class="menu-subtitle">{"Select an action to get started"}</p>
                </div>
                <div class="menu-grid">
                    {for MENU_BUTTONS.iter().map(|button| {
                        html! {
                            <button id={button.id} class={"menu-btn menu-btn--".to_owned() + button.variant}>
                                {button.label}
                            </button>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}
