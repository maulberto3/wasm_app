use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Reusable number input component with common event handling pattern
#[component]
pub fn NumberInput(
    id: &'static str,
    label: &'static str,
    #[prop(into)] value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    min: i32,
    max: i32,
) -> impl IntoView {
    view! {
        <div class="config-item">
            <label for=id>{label}</label>
            <input
                id=id
                type="number"
                min=min.to_string()
                max=max.to_string()
                value=value
                on:input=move |ev: leptos::ev::Event| {
                    if let Some(val) = ev.target() {
                        if let Some(input) = val.dyn_ref::<leptos::web_sys::HtmlInputElement>() {
                            on_change(input.value());
                        }
                    }
                }
            />
        </div>
    }
}
