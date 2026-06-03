use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: AttrValue,
    pub visible: bool,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    html! {
        <div class={ if props.visible { "toast toast--visible" } else { "toast" } }>
            <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13"
                viewBox="0 0 24 24" fill="none" stroke="currentColor"
                stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"/>
            </svg>
            { &props.message }
        </div>
    }
}
