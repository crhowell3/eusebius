use yew::prelude::*;

struct MenuButton {
    tab_label: &'static str,
    button_label: &'static str,
    id: &'static str,
    variant: &'static str,
}

const MENU_BUTTONS: &[MenuButton] = &[
    MenuButton {
        tab_label: "Members",
        button_label: "Update Members",
        id: "update-members",
        variant: "primary",
    },
    MenuButton {
        tab_label: "Works",
        button_label: "Update Works",
        id: "update-works",
        variant: "primary",
    },
    MenuButton {
        tab_label: "Deaths",
        button_label: "Update Deaths",
        id: "update-deaths",
        variant: "primary",
    },
    MenuButton {
        tab_label: "Baptisms",
        button_label: "Update Baptisms",
        id: "update-baptisms",
        variant: "primary",
    },
    MenuButton {
        tab_label: "Settings",
        button_label: "Settings",
        id: "settings",
        variant: "secondary",
    },
    MenuButton {
        tab_label: "About",
        button_label: "About Eusebius",
        id: "about",
        variant: "secondary",
    },
];

#[derive(Properties, PartialEq)]
pub struct MainMenuProps {
    pub open_tab: Callback<(String, String)>,
}

#[function_component(MainMenu)]
pub fn main_menu(props: &MainMenuProps) -> Html {
    let open_tab = props.open_tab.clone();

    html! {
        <div class="menu-backdrop">
            <div class="menu-panel">
                <div class="menu-header">
                    <h2 class="menu-title">{"Control Panel"}</h2>
                    <p class="menu-subtitle">{"Select an action to get started"}</p>
                </div>
                <div class="menu-grid">
                {for MENU_BUTTONS.iter().map(|button| {
                    let open_tab = open_tab.clone();
                    let id = button.id.to_string();
                    let label = button.tab_label.to_string();
                    html! {
                        <button
                            id={button.id}
                            class={"menu-btn menu-btn--".to_owned() + button.variant}
                            onclick={Callback::from(move |_: MouseEvent| {
                                open_tab.emit((id.clone(), label.clone()));
                            })}
                        >
                            {button.button_label}
                        </button>
                    }
                })}
                </div>
            </div>
        </div>
    }
}
