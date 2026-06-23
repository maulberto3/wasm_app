#[derive(Clone)]
pub struct AppState {
    pub title: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            title: "WASM CMA-ES Optimizer".to_string(),
        }
    }
}
