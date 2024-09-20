//! Defines the target architectures supported by the BF compiler

/// A target architecture supported by the BF compiler
pub enum Architecture {
    /// The aarch64 (ARM v8-A) architecture
    AARCH64,

    /// The AMD64 (x86-64) architecture
    AMD64,

    /// WebAssembly
    WASM,
}
