use gloo_timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::Toast;
use crate::utils::writeText;

#[derive(Properties, PartialEq)]
pub struct AboutProps {
    pub version: String,
    pub commit: String,
}

#[function_component(About)]
pub fn about(props: &AboutProps) -> Html {
    let toast_visible = use_state(|| false);

    let on_copy = {
        let toast_visible = toast_visible.clone();
        let text = format!(
            "eusebius {version}\nCommit: {commit}",
            version = props.version,
            commit = props.commit
        );
        Callback::from(move |_: MouseEvent| {
            toast_visible.set(true);

            let toast_visible = toast_visible.clone();
            Timeout::new(2_000, move || {
                toast_visible.set(false);
            })
            .forget();

            let text = text.clone();
            spawn_local(async move {
                writeText(&text).await;
            })
        })
    };

    html! {
        <div class="menu-backdrop">
            <div class="about-panel">
                <img src="assets/64x64.png" alt="Application Icon" style="margin-bottom: 14px"/>
                <h1 class="about-title">{format!("eusebius {}", props.version)}</h1>
                <div class="about-field">
                    <span class="about-label">{ "Author" }</span>
                    <span class="about-value">{ "Cameron Howell" }</span>
                </div>
                <div class="about-field">
                    <span class="about-label">{ "Commit" }</span>
                    <span class="about-value">{ &props.commit }</span>
                </div>
                <div class="about-btns">
                    <button class="about-btn" onclick={on_copy}>
                        <span>
                            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                                fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                            </svg>
                            { " Copy" }
                        </span>
                    </button>
                </div>

                <Toast message="Copied to clipboard!" visible={ *toast_visible } />
            </div>
        </div>
    }
}
