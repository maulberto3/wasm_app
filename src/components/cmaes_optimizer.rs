use leptos::prelude::*;

use crate::components::cmaes_state::OptimizerStateSignals;
use crate::components::cmaes_ui::{
    ConfigSection, ControlsSection, ParametersSection, StatsSection, StatusSection,
};

#[component]
pub fn CmaesOptimizer() -> impl IntoView {
    let state = OptimizerStateSignals::new();

    view! {
        <div class="cmaes-container">
            <div class="title-span">
                <h1>"CMA-ES Parameter Optimizer"</h1>
                <p class="subtitle">
                    "Watch thousands of parameters converge to the optimal solution in real-time"
                </p>
            </div>

            <div class="controls-left">
                <ConfigSection state=state />
                <ControlsSection state=state />
                <StatsSection state=state />
                <StatusSection state=state />
            </div>

            <div class="parameters-right">
                <ParametersSection state=state />
            </div>
        </div>
    }
}
