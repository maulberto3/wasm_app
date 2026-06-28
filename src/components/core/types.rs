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
