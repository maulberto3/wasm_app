use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use {wasm_bindgen::JsCast, web_sys};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectiveFunction {
    Sphere,    // x^2 sum
    Rastrigin, // more complex
    Ackley,    // more complex
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerState {
    Idle,
    Running,
    Paused,
    Complete,
}

#[component]
pub fn CmaesOptimizer() -> impl IntoView {
    // UI State - using RwSignal for Leptos 0.8.0
    let _objective_fn = RwSignal::new(ObjectiveFunction::Sphere);
    let num_dimensions = RwSignal::new(10);
    let population_size = RwSignal::new(15);
    let optimizer_state = RwSignal::new(OptimizerState::Idle);
    let iteration = RwSignal::new(0);
    let best_fitness = RwSignal::new(0.0);
    let elapsed_ms = RwSignal::new(0.0);
    let parameters = RwSignal::new(vec![0.0; 10]);
    let show_advanced = RwSignal::new(false);

    // Use objective_fn alias for handlers (will be used in next step)
    #[allow(unused_variables)]
    let objective_fn = _objective_fn;

    let handle_start = move |_| {
        optimizer_state.set(OptimizerState::Running);
        iteration.set(0);
        elapsed_ms.set(0.0);
        // TODO: Call CMA-ES here
    };

    let handle_reset = move |_| {
        optimizer_state.set(OptimizerState::Idle);
        iteration.set(0);
        best_fitness.set(0.0);
        elapsed_ms.set(0.0);
        let dims = num_dimensions.get();
        parameters.set(vec![0.0; dims]);
    };

    let handle_pause_continue = move |_| {
        let state = optimizer_state.get();
        if state == OptimizerState::Running {
            optimizer_state.set(OptimizerState::Paused);
        } else if state == OptimizerState::Paused {
            optimizer_state.set(OptimizerState::Running);
        }
    };

    #[cfg(feature = "hydrate")]
    let handle_function_change = move |ev: leptos::ev::Event| {
        if let Some(val) = ev.target() {
            if let Some(input) = val.dyn_ref::<web_sys::HtmlSelectElement>() {
                match input.value().as_str() {
                    "sphere" => objective_fn.set(ObjectiveFunction::Sphere),
                    "rastrigin" => objective_fn.set(ObjectiveFunction::Rastrigin),
                    "ackley" => objective_fn.set(ObjectiveFunction::Ackley),
                    _ => {}
                }
            }
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_function_change = move |_: leptos::ev::Event| {};

    #[cfg(feature = "hydrate")]
    let handle_dimensions_change = move |ev: leptos::ev::Event| {
        if let Some(val) = ev.target() {
            if let Some(input) = val.dyn_ref::<web_sys::HtmlInputElement>() {
                if let Ok(n) = input.value().parse::<usize>() {
                    num_dimensions.set(n);
                    parameters.set(vec![0.0; n]);
                }
            }
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_dimensions_change = move |_: leptos::ev::Event| {};

    #[cfg(feature = "hydrate")]
    let handle_popsize_change = move |ev: leptos::ev::Event| {
        if let Some(val) = ev.target() {
            if let Some(input) = val.dyn_ref::<web_sys::HtmlInputElement>() {
                if let Ok(n) = input.value().parse::<i32>() {
                    population_size.set(n);
                }
            }
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_popsize_change = move |_: leptos::ev::Event| {};

    view! {
        <div class="cmaes-container">
            <h1>"CMA-ES Parameter Optimizer"</h1>
            <p class="subtitle">
                "Watch thousands of parameters converge to the optimal solution in real-time"
            </p>

            <div class="config-section">
                <h2>"Configuration"</h2>

                <div class="config-grid">
                    <div class="config-item">
                        <label for="function-select">"Objective Function:"</label>
                        <select
                            id="function-select"
                            on:change=handle_function_change
                        >
                            <option value="sphere" selected>
                                {ObjectiveFunction::Sphere.display_name()}
                            </option>
                            <option value="rastrigin">
                                {ObjectiveFunction::Rastrigin.display_name()}
                            </option>
                            <option value="ackley">
                                {ObjectiveFunction::Ackley.display_name()}
                            </option>
                        </select>
                    </div>

                    <div class="config-item">
                        <label for="dimensions-input">"# of Dimensions:"</label>
                        <input
                            id="dimensions-input"
                            type="number"
                            min="2"
                            max="100"
                            value=move || num_dimensions.get().to_string()
                            on:input=handle_dimensions_change
                        />
                    </div>

                    <div class="config-item">
                        <label for="popsize-input">"Population Size:"</label>
                        <input
                            id="popsize-input"
                            type="number"
                            min="5"
                            max="200"
                            value=move || population_size.get().to_string()
                            on:input=handle_popsize_change
                        />
                    </div>
                </div>

                <div class="advanced-toggle">
                    <button
                        on:click=move |_| show_advanced.set(!show_advanced.get())
                        class="toggle-button"
                    >
                        {move || if show_advanced.get() { "▼" } else { "▶" }}
                        " Advanced Options"
                    </button>
                </div>

                {move || {
                    show_advanced.get().then(|| {
                        view! {
                            <div class="advanced-section">
                                <p>"Advanced options coming soon..."</p>
                            </div>
                        }
                    })
                }}
            </div>

            <div class="controls-section">
                <h2>"Controls"</h2>
                <div class="button-group">
                    <button
                        on:click=handle_start
                        disabled=move || optimizer_state.get() == OptimizerState::Running
                        class="btn btn-primary"
                    >
                        "▶ Start Optimization"
                    </button>
                    <button
                        on:click=handle_pause_continue
                        disabled=move || optimizer_state.get() == OptimizerState::Idle
                        class="btn btn-secondary"
                    >
                        {move || {
                            let state = optimizer_state.get();
                            if state == OptimizerState::Paused {
                                "⏸ Continue"
                            } else {
                                "⏸ Pause"
                            }
                        }}
                    </button>
                    <button
                        on:click=handle_reset
                        class="btn btn-tertiary"
                    >
                        "↻ Reset"
                    </button>
                </div>
            </div>

            <div class="stats-section">
                <h2>"Statistics"</h2>
                <div class="stats-grid">
                    <div class="stat-item">
                        <span class="stat-label">"Iteration:"</span>
                        <span class="stat-value">{move || iteration.get()}</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Best Fitness:"</span>
                        <span class="stat-value">{move || format!("{:.6}", best_fitness.get())}</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Elapsed Time:"</span>
                        <span class="stat-value">{move || format!("{:.2}ms", elapsed_ms.get())}</span>
                    </div>
                </div>
            </div>

            <div class="parameters-section">
                <h2>
                    "Parameters ("
                    {move || parameters.get().len()}
                    " variables)"
                </h2>

                <div class="parameter-grid">
                    {move || {
                        parameters
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(idx, param): (usize, f64)| {
                                view! {
                                    <div class="param" title={format!("x{}: {:.4}", idx, param)}>
                                        {format!("{:.4}", param)}
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>

            <div class="status-section">
                <p class="status-text">
                    {move || {
                        match optimizer_state.get() {
                            OptimizerState::Idle => "Ready. Click 'Start Optimization' to begin.".to_string(),
                            OptimizerState::Running => "Optimization running...".to_string(),
                            OptimizerState::Paused => "Optimization paused.".to_string(),
                            OptimizerState::Complete => "Optimization complete!".to_string(),
                        }
                    }}
                </p>
            </div>
        </div>
    }
}
