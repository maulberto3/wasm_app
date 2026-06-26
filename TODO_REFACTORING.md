# Code Simplification & Refactoring TODOs

## High Priority - UI/Event Handlers

- [ ] **Simplify Event Handlers in `cmaes_ui.rs`**
  - Currently using verbose nested pattern matching for event extraction
  - Create `event_helpers.rs` module with utility functions:
    - `get_select_value(ev: &Event) -> Option<String>`
    - `get_input_value(ev: &Event) -> Option<String>`
  - Replace all instances of:
    ```rust
    if let Some(val) = ev.target() {
        if let Some(input) = val.dyn_ref::<HtmlSelectElement>() {
            // ...
        }
    }
    ```
  - With simpler: `event_helpers::get_select_value(&ev)`

- [ ] **Use `event_target_value` from Leptos**
  - Leptos provides built-in helpers in `leptos_dom::helpers`
  - Replace manual event extraction with `event_target_value(&ev)`
  - Affected locations: function select, dimensions input, population size input

## Medium Priority - Code Cleanup

- [ ] **Fix hardcoded Title in `app.rs`**
  - Change from: `<Title text="Leptos + Axum Counter"/>`
  - To: `<Title text="CMA-ES Parameter Optimizer"/>`

- [ ] **Extract Component Props to Type Aliases** (if more components added)
  - Consider defining reusable prop types for section components

- [ ] **Add Comments to Complex State Flows**
  - Document the signal → UI → CSS pattern used for best individual highlighting
  - Explain pause/resume loop logic

## Lower Priority - Nice-to-haves

- [ ] **Create `src/event_helpers.rs` as utility module**
  - Centralize all event extraction logic
  - Make it reusable across components

- [ ] **Type Safety for Event Handlers**
  - Consider creating wrapper types for event handlers
  - Could prevent future bugs with type checking

## Architecture Notes

- Event handling is currently verbose due to web-sys's low-level nature
- Leptos exposes raw DOM APIs without heavy abstraction (good for control, requires verbosity)
- Best pattern so far: signal → conditional class → CSS (used for best individual highlighting)
