use gloo_timers::future::sleep;
use haru_cmaes::fitness::FitnessEvaluator;
use haru_cmaes::params::CmaesParams;
use haru_cmaes::params::CmaesParamsValidator;
use haru_cmaes::state::CmaesState;
use haru_cmaes::state::CmaesStateLogic;
use haru_cmaes::strategy::{CmaesAlgo, CmaesAlgoOptimizer};
use leptos::prelude::*;
use std::time::Duration;

use crate::components::core::{ObjectiveFunction, OptimizerState, OptimizerStateSignals};
use crate::components::objectives;

/// Handle the Start button click (async with proper CMA-ES ask/tell pattern)
pub async fn handle_start(state: OptimizerStateSignals) {
    let dims = state.num_dimensions.get_untracked();
    let obj_fn = state.objective_fn.get_untracked();
    let popsize = state.population_size.get_untracked();

    state.optimizer_state.set(OptimizerState::Running);
    state.iteration.set(0);
    state.best_fitness.set(f32::INFINITY);
    state.elapsed_ms.set(0.0);

    #[cfg(feature = "hydrate")]
    let start_time = js_sys::Date::now();

    // Initialize CMA-ES parameters
    let params = match CmaesParams::new()
        .and_then(|p| p.set_popsize(popsize))
        .and_then(|p| p.set_xstart(dims, 0.0))
    {
        Ok(p) => p,
        Err(_) => {
            state.optimizer_state.set(OptimizerState::Idle);
            return;
        }
    };

    // Create CMA-ES algorithm instance
    let cmaes = match CmaesAlgo::new(params) {
        Ok(algo) => algo,
        Err(_) => {
            state.optimizer_state.set(OptimizerState::Idle);
            return;
        }
    };

    // Initialize state
    let mut cmaes_state = match CmaesState::init_state(&cmaes.params) {
        Ok(s) => s,
        Err(_) => {
            state.optimizer_state.set(OptimizerState::Idle);
            return;
        }
    };

    // Create fitness evaluator using the crate's FitnessEvaluator trait
    let evaluator = objectives::get_evaluator(obj_fn, dims);

    let mut step: i32 = 1;
    let max_steps: i32 = 1000;

    loop {
        // Check if reset button was clicked - break if so
        if state.optimizer_state.get_untracked() == OptimizerState::Idle {
            break;
        }

        // Check if paused - skip iteration but keep loop running
        if state.optimizer_state.get_untracked() == OptimizerState::Paused {
            // Yield control to allow UI updates and checking for resume
            sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Ask for new population
        let mut pop = match cmaes.ask(&mut cmaes_state) {
            Ok(p) => p,
            Err(_) => break,
        };

        // Evaluate fitness using the crate's FitnessEvaluator trait
        let mut fitness = match evaluator.evaluate(&pop) {
            Ok(f) => f,
            Err(_) => break,
        };

        // Find the best individual index (minimum fitness for minimization)
        let best_idx = fitness
            .values
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx);

        if let Some(idx) = best_idx {
            state.best_individual_idx.set(Some(idx));
        }

        // Tell CMA-ES about the evaluation
        cmaes_state = match cmaes.tell(cmaes_state, &mut pop, &mut fitness) {
            Ok(new_state) => new_state,
            Err(_) => break,
        };

        // Update UI with current best
        if let Ok((_, best_fit)) = cmaes_state.get_best() {
            let best_fitness_val = best_fit.row(0)[0];
            state.best_fitness.set(best_fitness_val);
            state.iteration.set(step as u32);

            // Update parameters display: convert population to flat vec for display
            // pop.y is a matrix where each column is an individual
            let population_flat: Vec<f32> = pop.y.iter().copied().collect();
            state.parameters.set(population_flat);

            #[cfg(feature = "hydrate")]
            {
                let now = js_sys::Date::now();
                state.elapsed_ms.set((now - start_time) as f32);
            }
        }

        // Check if optimization is done
        if let Ok(true) = cmaes.is_done(&cmaes_state, step) {
            state.optimizer_state.set(OptimizerState::Complete);
            break;
        }

        // Safety limit
        if step >= max_steps {
            state.optimizer_state.set(OptimizerState::Idle);
            break;
        }

        step += 1;

        // Yield control to allow UI updates between iterations
        sleep(Duration::from_millis(0)).await;
    }

    #[cfg(feature = "hydrate")]
    {
        // Only update elapsed time if not reset (state is not Idle)
        if state.optimizer_state.get_untracked() != OptimizerState::Idle {
            let now = js_sys::Date::now();
            state.elapsed_ms.set((now - start_time) as f32);
        }
    }
}

/// Handle the Pause/Continue button click
pub fn handle_pause_continue(state: OptimizerStateSignals) {
    let current_state = state.optimizer_state.get_untracked();
    if current_state == OptimizerState::Running {
        state.optimizer_state.set(OptimizerState::Paused);
    } else if current_state == OptimizerState::Paused {
        state.optimizer_state.set(OptimizerState::Running);
    }
}

/// Handle the Reset button click
pub fn handle_reset(state: OptimizerStateSignals) {
    state.optimizer_state.set(OptimizerState::Idle);
    state.reset_optimization();
    state.reset_parameters();
}

/// Handle objective function selection change
pub fn handle_function_change(state: OptimizerStateSignals, value: String) {
    match value.as_str() {
        "sphere" => state.objective_fn.set(ObjectiveFunction::Sphere),
        "rastrigin" => state.objective_fn.set(ObjectiveFunction::Rastrigin),
        "ackley" => state.objective_fn.set(ObjectiveFunction::Ackley),
        _ => {}
    }
    state.optimizer_state.set(OptimizerState::Idle);
    state.reset_optimization();
    state.reset_parameters();
}

/// Handle number of dimensions change
pub fn handle_dimensions_change(state: OptimizerStateSignals, value: String) {
    if let Ok(n) = value.parse::<usize>() {
        state.num_dimensions.set(n);
        state.optimizer_state.set(OptimizerState::Idle);
        state.reset_optimization();
        state.reset_parameters();
    }
}

/// Handle population size change
pub fn handle_popsize_change(state: OptimizerStateSignals, value: String) {
    if let Ok(n) = value.parse::<i32>() {
        state.population_size.set(n);
        state.optimizer_state.set(OptimizerState::Idle);
        state.reset_optimization();
        state.reset_parameters();
    }
}

/// Spawn the async start handler (wrapper for use in event handlers)
pub fn start_optimization(_state: OptimizerStateSignals) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(handle_start(_state));
    }
}
