//! Options to control the optimizer

/// Options to control the optimizer
///
/// Uses the Builder Rust pattern
pub struct OptimizerOptions {
    /// Whether to coalesce adjacent matching instructions
    pub coalesce: bool,

    /// Whether to identify and flatten simple loops
    pub simple_loops: bool,

    /// Whether to apply vector scans
    pub scans: bool,
}

impl OptimizerOptions {
    /// Instantiates a new `OptimizerOptions` instance with defaults
    ///
    /// All optimizations are off by default
    pub fn new() -> Self {
        Self {
            coalesce: false,
            simple_loops: false,
            scans: false,
        }
    }

    /// Sets the `coalesce` field to the given value
    pub fn coalesce(mut self, coalesce: bool) -> Self {
        self.coalesce = coalesce;
        self
    }

    /// Sets the `simple_loops` field to the given value
    pub fn simple_loops(mut self, simple_loops: bool) -> Self {
        self.simple_loops = simple_loops;
        self
    }

    /// Sets the `scans` field to the given value
    pub fn scans(mut self, scans: bool) -> Self {
        self.scans = scans;
        self
    }
}
