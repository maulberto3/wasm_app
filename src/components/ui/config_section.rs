use leptos::prelude::*;

use crate::components::core::OptimizerStateSignals;
use crate::components::ui::render_helpers::render_function_options;
use crate::components::ui::{NumberInput, SelectField};

/// Configuration section component
#[component]
pub fn ConfigSection(state: OptimizerStateSignals) -> impl IntoView {
    view! {
        <div class="config-section">
            <h2>"Configuration"</h2>

            <div class="config-grid">
                <SelectField
                    id="function-select"
                    label="Objective Function:"
                    on_change=move |value: String| {
                        crate::components::cmaes_handlers::handle_function_change(state, value);
                    }
                >
                    {render_function_options()}
                </SelectField>

                <NumberInput
                    id="dimensions-input"
                    label="# of Dimensions:"
                    value=move || state.num_dimensions.get().to_string()
                    on_change=move |value: String| {
                        crate::components::cmaes_handlers::handle_dimensions_change(state, value);
                    }
                    min=2
                    max=100
                />

                <NumberInput
                    id="popsize-input"
                    label="Population Size:"
                    value=move || state.population_size.get().to_string()
                    on_change=move |value: String| {
                        crate::components::cmaes_handlers::handle_popsize_change(state, value);
                    }
                    min=5
                    max=200
                />
            </div>

            <div class="advanced-toggle">
                <button
                    on:click=move |_| state.show_advanced.set(!state.show_advanced.get())
                    class="toggle-button"
                >
                    {move || if state.show_advanced.get() { "▼" } else { "▶" }}
                    " Advanced Options"
                </button>
            </div>

            {move || {
                state.show_advanced.get().then(|| {
                    view! {
                        <div class="advanced-section">
                            <p>"Advanced options coming soon..."</p>
                        </div>
                    }
                })
            }}
        </div>
    }
}
