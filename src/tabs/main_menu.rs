use yew::prelude::*;

use crate::utils::invoke;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

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
        tab_label: "Table List",
        button_label: "Table List",
        id: "view-tables",
        variant: "primary",
    },
    MenuButton {
        tab_label: "Backup",
        button_label: "Perform Backup",
        id: "perform-backup",
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

    let on_exit = Callback::from(move |_: MouseEvent| {
        spawn_local(async move {
            let _ = invoke("exit_app", JsValue::UNDEFINED).await;
        });
    });

    html! {
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
            <div style="display: flex; flex-direction: column; padding: 1rem 0; width: 100%; justify-content: center; align-items: center">
                <button
                    id="exit-application"
                    class="menu-btn menu-btn--exit"
                    style="width: 10%; text-align: center"
                    onclick={ on_exit }
                >
                    {"Exit"}
                </button>
            </div>
        </div>
    }
}
