use crate::components::cmaes_helpers::OptimizerState;
use leptos::prelude::*;

/// Centralized state for the CMA-ES optimizer
#[derive(Clone, Copy)]
pub struct OptimizerStateSignals {
    pub objective_fn: RwSignal<crate::components::cmaes_helpers::ObjectiveFunction>,
    pub num_dimensions: RwSignal<usize>,
    pub population_size: RwSignal<i32>,
    pub optimizer_state: RwSignal<OptimizerState>,
    pub iteration: RwSignal<u32>,
    pub best_fitness: RwSignal<f32>,
    pub elapsed_ms: RwSignal<f32>,
    pub parameters: RwSignal<Vec<f32>>,
    pub show_advanced: RwSignal<bool>,
    pub best_individual_idx: RwSignal<Option<usize>>,
}

impl OptimizerStateSignals {
    pub fn new() -> Self {
        Self {
            objective_fn: RwSignal::new(
                crate::components::cmaes_helpers::ObjectiveFunction::Sphere,
            ),
            num_dimensions: RwSignal::new(10),
            population_size: RwSignal::new(15),
            optimizer_state: RwSignal::new(OptimizerState::Idle),
            iteration: RwSignal::new(0),
            best_fitness: RwSignal::new(0.0f32),
            elapsed_ms: RwSignal::new(0.0f32),
            parameters: RwSignal::new(vec![0.0f32; 10]),
            show_advanced: RwSignal::new(false),
            best_individual_idx: RwSignal::new(None),
        }
    }

    pub fn reset_optimization(&self) {
        self.iteration.set(0);
        self.best_fitness.set(0.0);
        self.elapsed_ms.set(0.0);
        self.best_individual_idx.set(None);
    }

    pub fn reset_parameters(&self) {
        let dims = self.num_dimensions.get();
        let pop_size = self.population_size.get() as usize;
        self.parameters.set(vec![0.0f32; dims * pop_size]);
    }
}

impl Default for OptimizerStateSignals {
    fn default() -> Self {
        Self::new()
    }
}
