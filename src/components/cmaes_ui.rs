use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::components::cmaes_helpers::{
    render_function_options, render_param_cell, OptimizerState,
};
use crate::components::cmaes_state::OptimizerStateSignals;

/// Configuration section component
#[component]
pub fn ConfigSection(state: OptimizerStateSignals) -> impl IntoView {
    view! {
        <div class="config-section">
            <h2>"Configuration"</h2>

            <div class="config-grid">
                <div class="config-item">
                    <label for="function-select">"Objective Function:"</label>
                    <select
                        id="function-select"
                        on:change=move |ev: leptos::ev::Event| {
                            if let Some(val) = ev.target() {
                                if let Some(input) = val.dyn_ref::<leptos::web_sys::HtmlSelectElement>() {
                                    crate::components::cmaes_handlers::handle_function_change(state, input.value());
                                }
                            }
                        }
                    >
                        {render_function_options()}
                    </select>
                </div>

                <div class="config-item">
                    <label for="dimensions-input">"# of Dimensions:"</label>
                    <input
                        id="dimensions-input"
                        type="number"
                        min="2"
                        max="100"
                        value=move || state.num_dimensions.get().to_string()
                        on:input=move |ev: leptos::ev::Event| {
                            if let Some(val) = ev.target() {
                                if let Some(input) = val.dyn_ref::<leptos::web_sys::HtmlInputElement>() {
                                    crate::components::cmaes_handlers::handle_dimensions_change(state, input.value());
                                }
                            }
                        }
                    />
                </div>

                <div class="config-item">
                    <label for="popsize-input">"Population Size:"</label>
                    <input
                        id="popsize-input"
                        type="number"
                        min="5"
                        max="200"
                        value=move || state.population_size.get().to_string()
                        on:input=move |ev: leptos::ev::Event| {
                            if let Some(val) = ev.target() {
                                if let Some(input) = val.dyn_ref::<leptos::web_sys::HtmlInputElement>() {
                                    crate::components::cmaes_handlers::handle_popsize_change(state, input.value());
                                }
                            }
                        }
                    />
                </div>
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

/// Controls section component
#[component]
pub fn ControlsSection(state: OptimizerStateSignals) -> impl IntoView {
    view! {
        <div class="controls-section">
            <h2>"Controls"</h2>
            <div class="button-group">
                <button
                    on:click=move |_| crate::components::cmaes_handlers::start_optimization(state)
                    disabled=move || state.optimizer_state.get() == OptimizerState::Running
                    class="btn btn-primary"
                >
                    "▶ Start Optimization"
                </button>
                <button
                    on:click=move |_| crate::components::cmaes_handlers::handle_pause_continue(state)
                    disabled=move || state.optimizer_state.get() == OptimizerState::Idle
                    class="btn btn-secondary"
                >
                    {move || {
                        let optimizer_state = state.optimizer_state.get();
                        if optimizer_state == OptimizerState::Paused {
                            "⏸ Continue"
                        } else {
                            "⏸ Pause"
                        }
                    }}
                </button>
                <button
                    on:click=move |_| crate::components::cmaes_handlers::handle_reset(state)
                    class="btn btn-tertiary"
                >
                    "↻ Reset"
                </button>
            </div>
        </div>
    }
}

/// Statistics section component
#[component]
pub fn StatsSection(state: OptimizerStateSignals) -> impl IntoView {
    view! {
        <div class="stats-section">
            <h2>"Statistics"</h2>
            <div class="stats-grid">
                <div class="stat-item">
                    <span class="stat-label">"Iteration:"</span>
                    <span class="stat-value">{move || state.iteration.get()}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">"Best Fitness:"</span>
                    <span class="stat-value">{move || format!("{:.6}", state.best_fitness.get())}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">"Elapsed Time:"</span>
                    <span class="stat-value">{move || format!("{:.2}ms", state.elapsed_ms.get())}</span>
                </div>
            </div>
        </div>
    }
}

/// Status section component
#[component]
pub fn StatusSection(state: OptimizerStateSignals) -> impl IntoView {
    view! {
        <div class="status-section">
            <p class="status-text">
                {move || {
                    match state.optimizer_state.get() {
                        OptimizerState::Idle => "Ready. Click 'Start Optimization' to begin.".to_string(),
                        OptimizerState::Running => "Optimization running...".to_string(),
                        OptimizerState::Paused => "Optimization paused.".to_string(),
                        OptimizerState::Complete => "Optimization complete!".to_string(),
                    }
                }}
            </p>
        </div>
    }
}

/// Parameters display section component
#[component]
pub fn ParametersSection(state: OptimizerStateSignals) -> impl IntoView {
    view! {
        <div class="parameters-section">
            <h2>
                "Population ("
                {move || state.population_size.get()}
                " individuals × "
                {move || state.num_dimensions.get()}
                " dimensions)"
            </h2>

            <div class="population-container">
                {move || {
                    let params = state.parameters.get();
                    let dims = state.num_dimensions.get();
                    let pop_size = state.population_size.get() as usize;
                    let best_idx = state.best_individual_idx.get();

                    (0..pop_size)
                        .map(|individual_idx| {
                            let is_best = best_idx == Some(individual_idx);
                            view! {
                                <div class="individual-row">
                                    <span
                                        class=move || {
                                            if is_best {
                                                "individual-label best-individual".to_string()
                                            } else {
                                                "individual-label".to_string()
                                            }
                                        }
                                    >
                                        {format!("Individual {}", individual_idx + 1)}
                                    </span>
                                    <div class="parameter-grid">
                                        {(0..dims)
                                            .map(|param_idx| {
                                                let global_idx = individual_idx * dims + param_idx;
                                                let value = params.get(global_idx).copied().unwrap_or(0.0f32);
                                                render_param_cell(param_idx, value)
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}
