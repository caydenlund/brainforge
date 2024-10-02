//! Options to control the optimizer

/// Options to control the optimizer
///
/// Uses the Builder Rust pattern
pub struct OptimizerOptions {
    /// Whether to coalesce adjacent matching instructions
    pub coalesce: bool,
}

impl OptimizerOptions {
    /// Instantiates a new `OptimizerOptions` instance with defaults
    ///
    /// All optimizations are off by default
    pub fn new() -> Self {
        Self {
            coalesce: false,
        }
    }

    /// Sets the `coalesce` field to the given value
    pub fn coalesce(mut self, coalesce: bool) -> Self {
        self.coalesce = coalesce;
        self
    }
}
