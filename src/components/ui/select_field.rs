use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Reusable select field component with common event handling pattern
#[component]
pub fn SelectField(
    id: &'static str,
    label: &'static str,
    on_change: impl Fn(String) + 'static,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="config-item">
            <label for=id>{label}</label>
            <select
                id=id
                on:change=move |ev: leptos::ev::Event| {
                    if let Some(val) = ev.target() {
                        if let Some(select) = val.dyn_ref::<leptos::web_sys::HtmlSelectElement>() {
                            on_change(select.value());
                        }
                    }
                }
            >
                {children()}
            </select>
        </div>
    }
}
