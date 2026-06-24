use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerState {
    Idle,
    Running,
    Paused,
    Complete,
}

#[derive(Clone, Debug)]
pub struct OptimizationResult {
    pub iteration: u32,
    pub best_fitness: f32,
    pub parameters: Vec<f32>,
    pub elapsed_ms: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectiveFunction {
    Sphere,
    Rastrigin,
    Ackley,
}

impl ObjectiveFunction {
    pub fn display_name(&self) -> &str {
        match self {
            ObjectiveFunction::Sphere => "Sphere (∑x²)",
            ObjectiveFunction::Rastrigin => "Rastrigin",
            ObjectiveFunction::Ackley => "Ackley",
        }
    }
}

/// Objective functions (f32)
pub fn sphere_function(x: &[f32]) -> f32 {
    x.iter().map(|xi| xi * xi).sum()
}

pub fn rastrigin_function(x: &[f32]) -> f32 {
    let n = x.len() as f32;
    let pi2 = 2.0 * std::f32::consts::PI;
    10.0 * n
        + x.iter()
            .map(|xi| xi * xi - 10.0 * (pi2 * xi).cos())
            .sum::<f32>()
}

pub fn ackley_function(x: &[f32]) -> f32 {
    let n = x.len() as f32;
    let sum_sq: f32 = x.iter().map(|xi| xi * xi).sum();
    let sum_cos: f32 = x
        .iter()
        .map(|xi| (2.0 * std::f32::consts::PI * xi).cos())
        .sum();
    -20.0 * (-0.2 * (sum_sq / n).sqrt()).exp() - (sum_cos / n).exp() + 20.0 + std::f32::consts::E
}

pub fn evaluate_objective(func: ObjectiveFunction, x: &[f32]) -> f32 {
    match func {
        ObjectiveFunction::Sphere => sphere_function(x),
        ObjectiveFunction::Rastrigin => rastrigin_function(x),
        ObjectiveFunction::Ackley => ackley_function(x),
    }
}

/// Run a single iteration of the optimization algorithm
pub fn run_optimization_iteration(
    _dims: usize,
    population_size: i32,
    obj_fn: ObjectiveFunction,
    iteration: u32,
    best_params: &[f32],
) -> (f32, Vec<f32>) {
    let mut best_this_step = evaluate_objective(obj_fn, best_params);
    let mut all_candidates = Vec::new();

    for _ in 0..population_size {
        let mut candidate = best_params.to_vec();
        let step_size = 1.0 / (1.0 + iteration as f32 * 0.1);

        for param in candidate.iter_mut() {
            let r = fastrand::f32();
            *param += (r - 0.5) * step_size;
        }

        let fitness = evaluate_objective(obj_fn, &candidate);
        if fitness < best_this_step {
            best_this_step = fitness;
        }
        all_candidates.extend(candidate);
    }

    (best_this_step, all_candidates)
}

// View helpers
pub fn render_function_options() -> impl IntoView {
    view! {
        <>
            <option value="sphere" selected>{ObjectiveFunction::Sphere.display_name()}</option>
            <option value="rastrigin">{ObjectiveFunction::Rastrigin.display_name()}</option>
            <option value="ackley">{ObjectiveFunction::Ackley.display_name()}</option>
        </>
    }
}

pub fn render_param_cell(idx: usize, param: f32) -> impl IntoView {
    view! {
        <div class="param" title={format!("x{}: {:.4}", idx, param)}>
            {format!("{:.4}", param)}
        </div>
    }
}
