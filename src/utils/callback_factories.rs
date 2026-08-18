use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Builds a callback for a plain text or select field, reading `input.value()` on
/// `InputEvent` and emitting `()
///
/// Usage:
/// ```
/// let on_change_some_field = make_field_callback(on_field_change.clone(), child_id, "some_field");
/// ```
pub fn make_field_callback<T: Clone + 'static>(
    on_field_change: Callback<(T, &'static str, String)>,
    id: T,
    field_name: &'static str,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            on_field_change.emit((id.clone(), field_name, input.value()));
        }
    })
}

pub fn make_bool_callback<T: Clone + 'static>(
    on_bool_change: Callback<(T, &'static str, bool)>,
    id: T,
    field_name: &'static str,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            on_bool_change.emit((id.clone(), field_name, input.checked()));
        }
    })
}

pub fn make_formatted_callback<T: Clone + 'static>(
    on_field_change: Callback<(T, &'static str, String)>,
    id: T,
    field_name: &'static str,
    formatter: fn(&str) -> String,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let formatted = formatter(&input.value());
            let _ = input.set_value(&formatted);
            on_field_change.emit((id.clone(), field_name, formatted));
        }
    })
}

pub fn make_form_formatted_callback<T, F>(
    dispatcher: UseReducerDispatcher<T>,
    field_name: &'static str,
    formatter: fn(&str) -> String,
    make_action: F,
) -> Callback<InputEvent>
where
    T: Reducible + 'static,
    F: Fn(String, String) -> T::Action + 'static,
{
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let formatted = formatter(&input.value());
            let _ = input.set_value(&formatted);
            dispatcher.dispatch(make_action(field_name.to_string(), formatted));
        }
    })
}
