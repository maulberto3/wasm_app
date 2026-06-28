use leptos::prelude::*;

use crate::components::core::{OptimizerState, OptimizerStateSignals};

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
