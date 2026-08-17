use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Builds a callback for a plain text or select field, reading `input.value()` on
/// `InputEvent` and emitting `(id, field_name, formatted_value)`.
///
/// Usage:
/// ```
/// let on_change_some_field = make_field_callback(on_field_change.clone(), child_id, "some_field");
/// ```
pub fn make_field_callback(
    on_field_change: Callback<(i64, &'static str, String)>,
    id: i64,
    field_name: &'static str,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
            on_field_change.emit((id, field_name, input.value()));
        }
    })
}

pub fn make_bool_callback(
    on_bool_change: Callback<(i64, &'static str, bool)>,
    id: i64,
    field_name: &'static str,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
            on_bool_change.emit((id, field_name, input.checked()));
        }
    })
}

pub fn make_formatted_callback(
    on_field_change: Callback<(i64, &'static str, String)>,
    id: i64,
    field_name: &'static str,
    formatter: fn(&str) -> String,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
            let formatted = formatter(&input.value());
            let _ = input.set_value(&formatted);
            on_field_change.emit((id, field_name, formatted));
        }
    })
}
