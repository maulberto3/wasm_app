#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use wasm_app::*;

#[wasm_bindgen_test]
fn test_add_function() {
    assert_eq!(add(2.0, 3.0), 5.0);
    assert_eq!(add(-1.0, 1.0), 0.0);
}

#[wasm_bindgen_test]
fn test_optimizer_initialization() {
    let optimizer = Optimizer::new(0.01, 1000);
    assert_eq!(optimizer.learning_rate(), 0.01);
    assert_eq!(optimizer.max_iterations(), 1000);
}

#[wasm_bindgen_test]
fn test_optimizer_optimize() {
    let optimizer = Optimizer::new(0.1, 100);
    let result = optimizer.optimize(1.0);
    assert!(result < 1.0); // Should reduce the value
}
