//! Options to control the optimizer

/// Options to control the optimizer
///
/// Uses the Builder Rust pattern
pub struct OptimizerOptions {
    /// Whether to coalesce adjacent matching instructions
    pub coalesce: bool,

    /// Whether to apply simple loops
    pub apply_simple_loops: bool,
}

impl OptimizerOptions {
    /// Instantiates a new `OptimizerOptions` instance with defaults
    ///
    /// All optimizations are off by default
    pub fn new() -> Self {
        Self {
            coalesce: false,
            apply_simple_loops: false,
        }
    }

    /// Sets the `coalesce` field to the given value
    pub fn coalesce(mut self, coalesce: bool) -> Self {
        self.coalesce = coalesce;
        self
    }

    /// Sets the `apply_simple_loops` field to the given value
    pub fn apply_simple_loops(mut self, apply_simple_loops: bool) -> Self {
        self.apply_simple_loops = apply_simple_loops;
        self
    }
}
