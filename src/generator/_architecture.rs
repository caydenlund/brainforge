//! Defines the target architectures supported by the BF compiler

/// A target architecture supported by the BF compiler
pub enum Architecture {
    /// The AArch64 (Armv8-A) architecture
    AArch64,

    /// The AMD64 (x86-64) architecture
    AMD64,

    /// WebAssembly
    WASM,
}
