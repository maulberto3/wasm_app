use crate::components::core::ObjectiveFunction;
use haru_cmaes::fitness::{MinOrMax, UserFitness};
use nalgebra::DVector;

/// Sphere function: ∑x²
/// A simple, convex benchmark function
fn sphere(x: &DVector<f32>) -> f32 {
    x.iter().map(|xi| xi * xi).sum()
}

/// Rastrigin function: multimodal benchmark with many local minima
fn rastrigin(x: &DVector<f32>) -> f32 {
    let n = x.len() as f32;
    let pi2 = 2.0 * std::f32::consts::PI;
    10.0 * n
        + x.iter()
            .map(|xi| xi * xi - 10.0 * (pi2 * xi).cos())
            .sum::<f32>()
}

/// Ackley function: multimodal with a narrow global optimum
fn ackley(x: &DVector<f32>) -> f32 {
    let n = x.len() as f32;
    let sum_sq: f32 = x.iter().map(|xi| xi * xi).sum();
    let sum_cos: f32 = x
        .iter()
        .map(|xi| (2.0 * std::f32::consts::PI * xi).cos())
        .sum();
    -20.0 * (-0.2 * (sum_sq / n).sqrt()).exp() - (sum_cos / n).exp() + 20.0 + std::f32::consts::E
}

/// Create a fitness evaluator for the specified objective function
/// Returns a UserFitness instance that implements FitnessEvaluator from the haru_cmaes crate
pub fn get_evaluator(
    obj_fn: ObjectiveFunction,
    dim: usize,
) -> UserFitness<fn(&DVector<f32>) -> f32> {
    match obj_fn {
        ObjectiveFunction::Sphere => UserFitness::new(sphere, dim, MinOrMax::Min),
        ObjectiveFunction::Rastrigin => UserFitness::new(rastrigin, dim, MinOrMax::Min),
        ObjectiveFunction::Ackley => UserFitness::new(ackley, dim, MinOrMax::Min),
    }
}
