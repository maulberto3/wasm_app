use crate::components::core::types::ObjectiveFunction;
use leptos::prelude::*;

/// View helpers for rendering objective function options
pub fn render_function_options() -> impl IntoView {
    view! {
        <>
            <option value="sphere" selected>{ObjectiveFunction::Sphere.display_name()}</option>
            <option value="rastrigin">{ObjectiveFunction::Rastrigin.display_name()}</option>
            <option value="ackley">{ObjectiveFunction::Ackley.display_name()}</option>
        </>
    }
}

/// Render a single parameter cell in the parameters grid
pub fn render_param_cell(idx: usize, param: f32) -> impl IntoView {
    view! {
        <div class="param" title={format!("x{}: {:.4}", idx, param)}>
            {format!("{:+.4}", param)}
        </div>
    }
}
