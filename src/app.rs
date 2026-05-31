use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;

use crate::tabs::{About, Baptisms, Deaths, MainMenu, MemberTabBody, Settings, WorkTabBody};

const COMMIT: &str = env!("GIT_COMMIT_HASH");

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: String,
    pub label: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn local_storage_get(key: &str) -> Option<String> {
    window()?.local_storage().ok()??.get_item(key).ok()?
}

fn local_storage_set(key: &str, value: &str) {
    if let Some(Ok(Some(storage))) = window().map(|w| w.local_storage()) {
        let _ = storage.set_item(key, value);
    }
}

fn apply_theme(dark: bool) {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(root) = doc.document_element() {
                let _ = root.set_attribute("data-theme", if dark { "dark" } else { "light" });
            }
        }
    }
}

fn tab_content(id: &str, _label: &str) -> Html {
    match id {
        "update-members" => html! { <MemberTabBody /> },
        "update-works" => html! { <WorkTabBody /> },
        "update-deaths" => html! { <Deaths /> },
        "update-baptisms" => html! { <Baptisms /> },
        "settings" => html! { <Settings /> },
        "about" => html! { <About version={env!("CARGO_PKG_VERSION")} commit={COMMIT} /> },
        _ => html! {},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let dark = use_state(|| local_storage_get("theme").as_deref() == Some("dark"));
    let tabs = use_state(|| {
        vec![Tab {
            id: "main".to_string(),
            label: "Main Menu".to_string(),
        }]
    });
    let active_tab = use_state(|| "main".to_string());

    let is_mounted = use_state(|| false);
    {
        let dark = dark.clone();
        let is_mounted = is_mounted.clone();
        use_effect_with(*dark, move |&is_dark| {
            apply_theme(is_dark);
            if *is_mounted {
                local_storage_set("theme", if is_dark { "dark" } else { "light" });
            } else {
                is_mounted.set(true);
            }
            || ()
        });
    }

    let toggle_dark = {
        let dark = dark.clone();
        Callback::from(move |_: MouseEvent| dark.set(!*dark))
    };

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
                <h1 class="app-title">{"Washington Street Members Database"}</h1>
                <button class="theme-toggle" onclick={toggle_dark}>
                    {match &*dark {
                        true => "☀ Light",
                        false => "☾ Dark",
                    }}
                </button>
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
                        let visible = tab.id == active;
                        html! {
                            <div
                                key={ tab.id.clone() }
                                class={ if visible { "tab-content tab-content--active" } else { "tab-content" } }
                            >
                                { tab_content(&tab.id, &tab.label) }
                            </div>
                        }
                    })
                }
            </main>
        </div>
    }
}
