use leptos::prelude::*;

use crate::components::core::OptimizerStateSignals;
use crate::components::ui::render_helpers::render_param_cell;

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
