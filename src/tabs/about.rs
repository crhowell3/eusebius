use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="menu-backdrop">
            <div class="menu-panel">
                <h2>{"Eusebius"}</h2>
                <p>{"Developed by Cameron Howell"}</p>
                <p>{"Application Version: v0.1.0"}</p>
            </div>
        </div>
    }
}
