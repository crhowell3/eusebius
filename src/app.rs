use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::tabs::{
    About, BackupsTabBody, BaptismsTabBody, DeathTabBody, MainMenu, MemberTabBody, SettingsTabBody,
    ViewTables, WorkTabBody,
};

use crate::utils::{apply_theme, invoke};

use shared::AppSettings;

const COMMIT: &str = env!("GIT_COMMIT_HASH");

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
struct TabPaneProps {
    pub id: AttrValue,
    pub active_id: AttrValue,
}

#[function_component(TabPane)]
fn tab_pane(props: &TabPaneProps) -> Html {
    let visible = props.id == props.active_id;
    let content = match props.id.as_str() {
        "update-members" => html! { <MemberTabBody /> },
        "update-works" => html! { <WorkTabBody /> },
        "update-deaths" => html! { <DeathTabBody /> },
        "update-baptisms" => html! { <BaptismsTabBody /> },
        "perform-backup" => html! { <BackupsTabBody /> },
        "view-tables" => html! { <ViewTables /> },
        "settings" => html! { <SettingsTabBody /> },
        "about" => html! { <About version={env!("CARGO_PKG_VERSION")} commit={COMMIT} /> },
        _ => html! {},
    };

    html! {
        <div class={ if visible { "tab-content tab-content--active" } else { "tab-content" } }>
            { content }
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let tabs = use_state(|| {
        vec![Tab {
            id: "main".to_string(),
            label: "Main Menu".to_string(),
        }]
    });
    let active_tab = use_state(|| "main".to_string());

    use_effect_with((), move |_| {
        spawn_local(async move {
            let result = invoke("load_settings", JsValue::UNDEFINED).await;
            if let Ok(settings) = from_value::<AppSettings>(result) {
                apply_theme(&settings.theme);
            }
        });

        || ()
    });

    let open_tab = {
        let tabs = tabs.clone();
        let active_tab = active_tab.clone();
        Callback::from(move |(id, label): (String, String)| {
            if !(*tabs).iter().any(|t| t.id == id) {
                let mut next = (*tabs).clone();
                next.push(Tab {
                    id: id.clone(),
                    label,
                });
                tabs.set(next);
            }
            active_tab.set(id);
        })
    };

    let close_tab = {
        let tabs = tabs.clone();
        let active_tab = active_tab.clone();
        Callback::from(move |id: String| {
            let remaining: Vec<Tab> = (*tabs).iter().filter(|t| t.id != id).cloned().collect();
            if *active_tab == id {
                if let Some(last) = remaining.last() {
                    active_tab.set(last.id.clone());
                }
            }
            tabs.set(remaining);
        })
    };

    let switch_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |id: String| active_tab.set(id))
    };

    let active = (*active_tab).clone();
    let show_main = *active_tab == "main";

    html! {
        <div class="app-wrapper">
            <header class="app-header">
                <h1 class="app-title">{"Eusebius Database Manager"}</h1>
            </header>

            <div class="tab-bar">
                { for (*tabs).iter().map(|tab| {
                    let is_active = tab.id == active;
                    let class = if is_active { "tab tab--active" } else { "tab" };

                    let on_click = {
                        let id = tab.id.clone();
                        let switch_tab = switch_tab.clone();
                        Callback::from(move |_: MouseEvent| switch_tab.emit(id.clone()))
                    };

                    let close_btn = if tab.id != "main" {
                        let id = tab.id.clone();
                        let close_tab = close_tab.clone();
                        let label = tab.label.clone();
                        html! {
                            <span
                                class="tab-close"
                                aria-label={ format!("Close {}", label) }
                                onclick={ Callback::from(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    close_tab.emit(id.clone());
                                }) }
                            >
                                { "×" }
                            </span>
                        }
                    } else {
                        html! {}
                    };

                    html! {
                        <button key={ tab.id.clone() } class={class} onclick={on_click}>
                            { &tab.label }
                            { close_btn }
                        </button>
                    }
                }) }
            </div>

            <main class="app-main">
                if show_main {
                    <MainMenu open_tab={open_tab} />
                }

                {
                    for (*tabs).iter().filter(|t| t.id != "main").map(|tab| {
                        html! {
                            <TabPane
                                key={ tab.id.clone() }
                                id={ tab.id.clone() }
                                active_id={ (*active_tab).clone() }
                            />
                        }
                    })
                }
            </main>
        </div>
    }
}
