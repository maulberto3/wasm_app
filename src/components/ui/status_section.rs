use leptos::prelude::*;

use crate::components::core::{OptimizerState, OptimizerStateSignals};

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
