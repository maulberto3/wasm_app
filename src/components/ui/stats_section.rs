use leptos::prelude::*;

use crate::components::core::OptimizerStateSignals;

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
