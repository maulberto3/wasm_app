use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerState {
    Idle,
    Running,
    Paused,
    Complete,
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
            {format!("{:+.4}", param)}
        </div>
    }
}
