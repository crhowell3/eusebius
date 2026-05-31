use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::writeText;

#[derive(Properties, PartialEq)]
pub struct AboutProps {
    pub version: String,
    pub commit: String,
}

#[function_component(About)]
pub fn about(props: &AboutProps) -> Html {
    let on_copy = {
        let text = format!(
            "eusebius {version}\nCommit: {commit}",
            version = props.version,
            commit = props.commit
        );
        Callback::from(move |_: MouseEvent| {
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
                    <button class="about-btn" onclick={on_copy}>{ "Copy" }</button>
                </div>
            </div>
        </div>
    }
}
